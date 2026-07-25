use std::sync::Arc;

use crate::{eventbus::eventbus::Eventbus, model::event::Event};
use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use tokio::sync::broadcast::Receiver;

#[derive(Clone)]
pub struct WebSocketClient {
    pub eventbus: Arc<Eventbus>,
}

impl WebSocketClient {
    pub fn new(eventbus: Arc<Eventbus>) -> Self {
        Self { eventbus }
    }

    pub async fn setup_ws(
        State(websocket_client): State<Arc<WebSocketClient>>,
        ws: WebSocketUpgrade,
    ) -> impl IntoResponse {
        let mut receiver = websocket_client.eventbus.subscribe();
        ws.on_upgrade(move |socket| async move {
            Self::send(socket, &mut receiver).await;
        })
    }

    pub async fn send(mut socket: WebSocket, receiver: &mut Receiver<Event>) {
        loop {
            match receiver.recv().await {
                Ok(el) => socket
                    .send(axum::extract::ws::Message::Text(
                        serde_json::to_string(&el).unwrap().into(),
                    ))
                    .await
                    .unwrap(),
                Err(_) => break,
            }
        }
    }
}
