use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures::stream::StreamExt;
use prost::Message as ProstMessage;

pub mod mocap {
    include!(concat!(env!("OUT_DIR"), "/mocap.rs"));
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Ядро запущено на {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("Клиент подключен");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Binary(bytes) => match mocap::MocapFrame::decode(bytes) {
                Ok(frame) => {
                    println!(
                        "Тип: {}, точек: {}",
                        frame.skeleton_type,
                        frame.landmarks.len()
                    );
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
