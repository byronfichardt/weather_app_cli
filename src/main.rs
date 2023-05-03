use std::net::SocketAddr;
use axum::{routing::get, Router};

mod controllers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(controllers::home::home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
