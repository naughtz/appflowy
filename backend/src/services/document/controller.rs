use crate::services::{
    document::{
        persistence::{create_doc, read_doc},
        ws_actor::{DocumentWebSocketActor, WSActorMessage},
    },
    web_socket::{WSClientData, WebSocketReceiver},
};

use crate::context::FlowyPersistence;
use backend_service::errors::ServerError;
use flowy_collaboration::{
    core::sync::{DocumentPersistence, ServerDocumentManager},
    entities::{
        doc::{CreateDocParams, Doc},
        revision::{RepeatedRevision, Revision},
    },
    errors::CollaborateError,
    protobuf::DocIdentifier,
};
use lib_infra::future::FutureResultSend;

use std::{convert::TryInto, sync::Arc};
use tokio::sync::{mpsc, oneshot};

pub fn make_document_ws_receiver(persistence: Arc<FlowyPersistence>) -> Arc<DocumentWebSocketReceiver> {
    let document_persistence = Arc::new(DocumentPersistenceImpl(persistence.clone()));
    let document_manager = Arc::new(ServerDocumentManager::new(document_persistence));

    let (ws_sender, rx) = tokio::sync::mpsc::channel(100);
    let actor = DocumentWebSocketActor::new(rx, document_manager);
    tokio::task::spawn(actor.run());

    Arc::new(DocumentWebSocketReceiver::new(persistence, ws_sender))
}

pub struct DocumentWebSocketReceiver {
    ws_sender: mpsc::Sender<WSActorMessage>,
    persistence: Arc<FlowyPersistence>,
}

impl DocumentWebSocketReceiver {
    pub fn new(persistence: Arc<FlowyPersistence>, ws_sender: mpsc::Sender<WSActorMessage>) -> Self {
        Self { ws_sender, persistence }
    }
}

impl WebSocketReceiver for DocumentWebSocketReceiver {
    fn receive(&self, data: WSClientData) {
        let (ret, rx) = oneshot::channel();
        let sender = self.ws_sender.clone();
        let persistence = self.persistence.clone();

        actix_rt::spawn(async move {
            let msg = WSActorMessage::ClientData {
                client_data: data,
                ret,
                persistence,
            };
            match sender.send(msg).await {
                Ok(_) => {},
                Err(e) => log::error!("{}", e),
            }
            match rx.await {
                Ok(_) => {},
                Err(e) => log::error!("{:?}", e),
            };
        });
    }
}

struct DocumentPersistenceImpl(Arc<FlowyPersistence>);
impl DocumentPersistence for DocumentPersistenceImpl {
    // fn update_doc(&self, doc_id: &str, rev_id: i64, delta: RichTextDelta) ->
    // FutureResultSend<(), CollaborateError> {     let pg_pool =
    // self.0.pg_pool();     let mut params = ResetDocumentParams::new();
    //     let doc_json = delta.to_json();
    //     params.set_doc_id(doc_id.to_string());
    //     params.set_data(doc_json);
    //     params.set_rev_id(rev_id);
    //
    //     FutureResultSend::new(async move {
    //         let _ = update_doc(&pg_pool, params)
    //             .await
    //             .map_err(server_error_to_collaborate_error)?;
    //         Ok(())
    //     })
    // }

    fn read_doc(&self, doc_id: &str) -> FutureResultSend<Doc, CollaborateError> {
        let params = DocIdentifier {
            doc_id: doc_id.to_string(),
            ..Default::default()
        };
        let persistence = self.0.clone();
        FutureResultSend::new(async move {
            let mut pb_doc = read_doc(&persistence, params)
                .await
                .map_err(server_error_to_collaborate_error)?;
            let doc = (&mut pb_doc)
                .try_into()
                .map_err(|e| CollaborateError::internal().context(e))?;
            Ok(doc)
        })
    }

    fn create_doc(&self, revision: Revision) -> FutureResultSend<Doc, CollaborateError> {
        let kv_store = self.0.kv_store();
        FutureResultSend::new(async move {
            let doc: Doc = revision.clone().try_into()?;
            let doc_id = revision.doc_id.clone();
            let revisions = RepeatedRevision { items: vec![revision] };

            let params = CreateDocParams { id: doc_id, revisions };
            let pb_params: flowy_collaboration::protobuf::CreateDocParams = params.try_into().unwrap();
            let _ = create_doc(&kv_store, pb_params)
                .await
                .map_err(server_error_to_collaborate_error)?;
            Ok(doc)
        })
    }
}

fn server_error_to_collaborate_error(error: ServerError) -> CollaborateError {
    if error.is_record_not_found() {
        CollaborateError::record_not_found()
    } else {
        CollaborateError::internal().context(error)
    }
}