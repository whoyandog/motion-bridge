use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures::stream::StreamExt;

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
        if let Message::Text(text) = msg {
            println!("Кадр: {}", text);
        } else if let Message::Close(_) = msg {
            println!("Клиент отключился");
            break;
        }
    }
}
