pub mod mocap {
    include!(concat!(env!("OUT_DIR"), "/mocap.rs"));
}

mod network;

#[tokio::main]
async fn main() {
    let app = network::ws_server::create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Ядро запущено на {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
