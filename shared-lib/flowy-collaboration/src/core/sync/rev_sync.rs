use crate::{
    core::document::Document,
    entities::ws::{WsDataType, WsDocumentData},
};
use bytes::Bytes;
use lib_ot::{
    core::OperationTransformable,
    errors::OTError,
    protobuf::RevId,
    revision::{RevType, Revision, RevisionRange},
    rich_text::RichTextDelta,
};
use parking_lot::RwLock;
use protobuf::Message;
use std::{
    cmp::Ordering,
    convert::TryInto,
    sync::{
        atomic::{AtomicI64, Ordering::SeqCst},
        Arc,
    },
    time::Duration,
};
use tokio::sync::mpsc;

pub enum SynchronizerCommand {
    Pull(WsDocumentData),
    Push(WsDocumentData),
    Ack(WsDocumentData),
    SaveRevision(Revision),
}

pub type CommandReceiver = Arc<dyn Fn(SynchronizerCommand)>;

pub struct RevisionSynchronizer {
    pub doc_id: String,
    pub rev_id: AtomicI64,
    document: Arc<RwLock<Document>>,
    command_receiver: CommandReceiver,
}

impl RevisionSynchronizer {
    pub fn new(
        doc_id: &str,
        rev_id: i64,
        document: Arc<RwLock<Document>>,
        command_receiver: CommandReceiver,
    ) -> RevisionSynchronizer {
        RevisionSynchronizer {
            doc_id: doc_id.to_string(),
            rev_id: AtomicI64::new(rev_id),
            document,
            command_receiver,
        }
    }

    pub fn new_conn(&self, rev_id: i64) {
        let cur_rev_id = self.rev_id.load(SeqCst);
        match cur_rev_id.cmp(&rev_id) {
            Ordering::Less => {
                let msg = mk_pull_message(&self.doc_id, next(cur_rev_id), rev_id);
                self.send_command(SynchronizerCommand::Pull(msg));
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                let doc_delta = self.document.read().delta().clone();
                let revision = self.mk_revision(rev_id, doc_delta);
                let data = mk_push_message(&self.doc_id, revision);
                self.send_command(SynchronizerCommand::Push(data));
            },
        }
    }

    pub fn apply_revision(&self, revision: Revision) -> Result<(), OTError> {
        let cur_rev_id = self.rev_id.load(SeqCst);
        match cur_rev_id.cmp(&revision.rev_id) {
            Ordering::Less => {
                let next_rev_id = next(cur_rev_id);
                if cur_rev_id == revision.base_rev_id || next_rev_id == revision.base_rev_id {
                    // The rev is in the right order, just compose it.
                    let _ = self.compose_revision(&revision)?;
                    self.send_command(SynchronizerCommand::Ack(mk_acked_message(&revision)));
                    self.send_command(SynchronizerCommand::SaveRevision(revision));
                } else {
                    // The server document is outdated, pull the missing revision from the client.
                    let msg = mk_pull_message(&self.doc_id, next_rev_id, revision.rev_id);
                    self.send_command(SynchronizerCommand::Pull(msg));
                }
            },
            Ordering::Equal => {
                // Do nothing
                log::warn!("Applied revision rev_id is the same as cur_rev_id");
            },
            Ordering::Greater => {
                // The client document is outdated. Transform the client revision delta and then
                // send the prime delta to the client. Client should compose the this prime
                // delta.
                let cli_revision = self.transform_revision(&revision)?;
                let data = mk_push_message(&self.doc_id, cli_revision);
                self.send_command(SynchronizerCommand::Push(data));
            },
        }
        Ok(())
    }

    fn compose_revision(&self, revision: &Revision) -> Result<(), OTError> {
        let delta = RichTextDelta::from_bytes(&revision.delta_data)?;
        let _ = self.compose_delta(delta)?;
        let _ = self.rev_id.fetch_update(SeqCst, SeqCst, |_e| Some(revision.rev_id));
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self, revision))]
    fn transform_revision(&self, revision: &Revision) -> Result<Revision, OTError> {
        let cli_delta = RichTextDelta::from_bytes(&revision.delta_data)?;
        let (cli_prime, server_prime) = self.document.read().delta().transform(&cli_delta)?;

        let _ = self.compose_delta(server_prime)?;
        let cli_revision = self.mk_revision(revision.rev_id, cli_prime);
        Ok(cli_revision)
    }

    fn send_command(&self, command: SynchronizerCommand) { (self.command_receiver)(command); }

    #[tracing::instrument(
        level = "debug",
        skip(self, delta),
        fields(
        revision_delta = %delta.to_json(),
        result,
        )
    )]
    fn compose_delta(&self, delta: RichTextDelta) -> Result<(), OTError> {
        if delta.is_empty() {
            log::warn!("Composed delta is empty");
        }

        match self.document.try_write_for(Duration::from_millis(300)) {
            None => log::error!("Failed to acquire write lock of document"),
            Some(mut write_guard) => {
                let _ = write_guard.compose_delta(delta);
                tracing::Span::current().record("result", &write_guard.to_json().as_str());
            },
        }
        Ok(())
    }

    fn mk_revision(&self, base_rev_id: i64, delta: RichTextDelta) -> Revision {
        let delta_data = delta.to_bytes().to_vec();
        let md5 = md5(&delta_data);
        Revision {
            base_rev_id,
            rev_id: self.rev_id.load(SeqCst),
            delta_data,
            md5,
            doc_id: self.doc_id.to_string(),
            ty: RevType::Remote,
            user_id: "".to_string(),
        }
    }
}

fn mk_push_message(doc_id: &str, revision: Revision) -> WsDocumentData {
    let bytes: Bytes = revision.try_into().unwrap();
    WsDocumentData {
        doc_id: doc_id.to_string(),
        ty: WsDataType::PushRev,
        data: bytes.to_vec(),
    }
}

fn mk_pull_message(doc_id: &str, from_rev_id: i64, to_rev_id: i64) -> WsDocumentData {
    let range = RevisionRange {
        doc_id: doc_id.to_string(),
        start: from_rev_id,
        end: to_rev_id,
    };

    let bytes: Bytes = range.try_into().unwrap();
    WsDocumentData {
        doc_id: doc_id.to_string(),
        ty: WsDataType::PullRev,
        data: bytes.to_vec(),
    }
}

fn mk_acked_message(revision: &Revision) -> WsDocumentData {
    // let mut wtr = vec![];
    // let _ = wtr.write_i64::<BigEndian>(revision.rev_id);
    let mut rev_id = RevId::new();
    rev_id.set_value(revision.rev_id);
    let data = rev_id.write_to_bytes().unwrap();

    WsDocumentData {
        doc_id: revision.doc_id.clone(),
        ty: WsDataType::Acked,
        data,
    }
}

#[inline]
fn next(rev_id: i64) -> i64 { rev_id + 1 }

#[inline]
fn md5<T: AsRef<[u8]>>(data: T) -> String {
    let md5 = format!("{:x}", md5::compute(data));
    md5
}