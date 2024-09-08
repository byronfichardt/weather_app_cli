mod note;

use axum::routing::post;
use axum::{body::Body, http::Method, routing::get, Extension, Router};
use note::get_notes;
use note::create_note;
use diesel::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes(connection: Pool<ConnectionManager<MysqlConnection>>) -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/notes", get(get_notes))
        .route("/api/notes", post(create_note))
        .layer(Extension(connection.clone()))
        .layer(cors);

    app
}
