use crate::mocap;
use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    routing::get,
};
use futures::stream::StreamExt;
use prost::Message as ProstMessage;
use std::sync::Arc;
use tokio::sync::watch;

type SharedState = Arc<watch::Sender<Option<mocap::MocapFrame>>>;

pub fn create_router(tx: watch::Sender<Option<mocap::MocapFrame>>) -> Router {
    let state = Arc::new(tx);
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, tx: SharedState) {
    println!("Клиент подключен по WS (TCP)");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Binary(bytes) => match mocap::MocapFrame::decode(bytes) {
                Ok(frame) => {
                    let _ = tx.send(Some(frame));
                }
                Err(e) => {
                    eprintln!("Ошибка парсига protobuf: {}", e);
                }
            },
            Message::Close(_) => {
                println!("Клиент отключен");
                break;
            }
            _ => {}
        }
    }
}
