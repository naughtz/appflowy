use crate::errors::UserError;

use lib_infra::{entities::network_state::NetworkType, future::ResultFuture};
use lib_ws::{WsConnectState, WsController, WsMessage, WsMessageHandler, WsSender};
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::{broadcast, broadcast::Receiver};

pub trait FlowyWebSocket: Send + Sync {
    fn start_connect(&self, addr: String) -> ResultFuture<(), UserError>;
    fn conn_state_subscribe(&self) -> broadcast::Receiver<WsConnectState>;
    fn reconnect(&self, count: usize) -> ResultFuture<(), UserError>;
    fn add_handler(&self, handler: Arc<dyn WsMessageHandler>) -> Result<(), UserError>;
    fn ws_sender(&self) -> Result<Arc<dyn FlowyWsSender>, UserError>;
}

pub trait FlowyWsSender: Send + Sync {
    fn send(&self, msg: WsMessage) -> Result<(), UserError>;
}

pub struct WsManager {
    inner: Arc<dyn FlowyWebSocket>,
    connect_type: RwLock<NetworkType>,
}

impl WsManager {
    pub fn new() -> Self { WsManager::default() }

    pub async fn start(&self, addr: String) -> Result<(), UserError> {
        self.listen_on_websocket();
        let _ = self.inner.start_connect(addr).await?;
        Ok(())
    }

    pub fn update_network_type(&self, new_type: &NetworkType) {
        let old_type = self.connect_type.read().clone();
        if &old_type != new_type {
            log::debug!("Connect type switch from {:?} to {:?}", old_type, new_type);
            match (old_type.is_connect(), new_type.is_connect()) {
                (false, true) => {
                    let ws_controller = self.inner.clone();
                    tokio::spawn(async move { retry_connect(ws_controller, 100).await });
                },
                (true, false) => {
                    //
                },
                _ => {},
            }

            *self.connect_type.write() = new_type.clone();
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn listen_on_websocket(&self) {
        let mut notify = self.inner.conn_state_subscribe();
        let ws = self.inner.clone();
        let _ = tokio::spawn(async move {
            loop {
                match notify.recv().await {
                    Ok(state) => {
                        tracing::info!("Websocket state changed: {}", state);
                        match state {
                            WsConnectState::Init => {},
                            WsConnectState::Connected => {},
                            WsConnectState::Connecting => {},
                            WsConnectState::Disconnected => retry_connect(ws.clone(), 100).await,
                        }
                    },
                    Err(e) => {
                        log::error!("Websocket state notify error: {:?}", e);
                        break;
                    },
                }
            }
        });
    }

    pub fn state_subscribe(&self) -> broadcast::Receiver<WsConnectState> { self.inner.conn_state_subscribe() }

    pub fn add_handler(&self, handler: Arc<dyn WsMessageHandler>) -> Result<(), UserError> {
        let _ = self.inner.add_handler(handler)?;
        Ok(())
    }

    pub fn ws_sender(&self) -> Result<Arc<dyn FlowyWsSender>, UserError> {
        //
        self.inner.ws_sender()
    }
}

async fn retry_connect(ws: Arc<dyn FlowyWebSocket>, count: usize) {
    match ws.reconnect(count).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("websocket connect failed: {:?}", e);
        },
    }
}

impl std::default::Default for WsManager {
    fn default() -> Self {
        let ws: Arc<dyn FlowyWebSocket> = if cfg!(feature = "http_server") {
            Arc::new(Arc::new(WsController::new()))
        } else {
            Arc::new(Arc::new(mock::MockWebSocket::new()))
        };

        WsManager {
            inner: ws,
            connect_type: RwLock::new(NetworkType::default()),
        }
    }
}

impl FlowyWebSocket for Arc<WsController> {
    fn start_connect(&self, addr: String) -> ResultFuture<(), UserError> {
        let cloned_ws = self.clone();
        ResultFuture::new(async move {
            let _ = cloned_ws.start(addr).await?;
            Ok(())
        })
    }

    fn conn_state_subscribe(&self) -> Receiver<WsConnectState> { self.state_subscribe() }

    fn reconnect(&self, count: usize) -> ResultFuture<(), UserError> {
        let cloned_ws = self.clone();
        ResultFuture::new(async move {
            let _ = cloned_ws.retry(count).await?;
            Ok(())
        })
    }

    fn add_handler(&self, handler: Arc<dyn WsMessageHandler>) -> Result<(), UserError> {
        let _ = self.add_handler(handler)?;
        Ok(())
    }

    fn ws_sender(&self) -> Result<Arc<dyn FlowyWsSender>, UserError> {
        let sender = self.sender()?;
        Ok(sender)
    }
}

impl FlowyWsSender for WsSender {
    fn send(&self, msg: WsMessage) -> Result<(), UserError> {
        let _ = self.send_msg(msg)?;
        Ok(())
    }
}

// #[cfg(not(feature = "http_server"))]
mod mock {
    use crate::{
        errors::UserError,
        services::user::ws_manager::{FlowyWebSocket, FlowyWsSender},
    };
    use dashmap::DashMap;
    use lib_infra::future::ResultFuture;
    use lib_ws::{WsConnectState, WsMessage, WsMessageHandler, WsModule};
    use std::sync::Arc;
    use tokio::sync::{broadcast, broadcast::Receiver};

    pub struct MockWebSocket {
        handlers: DashMap<WsModule, Arc<dyn WsMessageHandler>>,
        state_sender: broadcast::Sender<WsConnectState>,
        ws_sender: broadcast::Sender<WsMessage>,
    }

    impl std::default::Default for MockWebSocket {
        fn default() -> Self {
            let (state_sender, _) = broadcast::channel(16);
            let (ws_sender, _) = broadcast::channel(16);
            MockWebSocket {
                handlers: DashMap::new(),
                state_sender,
                ws_sender,
            }
        }
    }

    impl MockWebSocket {
        pub fn new() -> MockWebSocket { MockWebSocket::default() }
    }

    impl FlowyWebSocket for Arc<MockWebSocket> {
        fn start_connect(&self, _addr: String) -> ResultFuture<(), UserError> {
            let mut ws_receiver = self.ws_sender.subscribe();
            let cloned_ws = self.clone();
            tokio::spawn(async move {
                while let Ok(message) = ws_receiver.recv().await {
                    match cloned_ws.handlers.get(&message.module) {
                        None => log::error!("Can't find any handler for message: {:?}", message),
                        Some(handler) => handler.receive_message(message.clone()),
                    }
                }
            });

            ResultFuture::new(async { Ok(()) })
        }

        fn conn_state_subscribe(&self) -> Receiver<WsConnectState> { self.state_sender.subscribe() }

        fn reconnect(&self, _count: usize) -> ResultFuture<(), UserError> { ResultFuture::new(async { Ok(()) }) }

        fn add_handler(&self, handler: Arc<dyn WsMessageHandler>) -> Result<(), UserError> {
            let source = handler.source();
            if self.handlers.contains_key(&source) {
                log::error!("WsSource's {:?} is already registered", source);
            }
            self.handlers.insert(source, handler);
            Ok(())
        }

        fn ws_sender(&self) -> Result<Arc<dyn FlowyWsSender>, UserError> { Ok(Arc::new(self.ws_sender.clone())) }
    }

    impl FlowyWsSender for broadcast::Sender<WsMessage> {
        fn send(&self, _msg: WsMessage) -> Result<(), UserError> {
            // let _ = self.send(msg).unwrap();
            Ok(())
        }
    }
}