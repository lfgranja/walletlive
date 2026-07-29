use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = Router::new().route("/", get(hello_world));

    axum::serve(listener, router).await.unwrap();
}

async fn hello_world() -> &'static str {
    "hello, world!"
}
