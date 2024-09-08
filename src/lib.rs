use std::net::SocketAddr;

use db::get_connection_pool;
use routes::create_routes;

pub mod db;
pub mod routes;

pub async fn run() {
    let connection = get_connection_pool();

    let app = create_routes(connection.await);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
