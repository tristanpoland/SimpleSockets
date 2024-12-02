use axum::routing::get;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};


fn on_connect(socket: SocketRef) {}

async fn server() {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .layer(layer);

    println!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main(){
    server().await;
}