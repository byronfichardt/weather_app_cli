use std::fs;

use axum::response::Html;

use note_app::view;

pub async fn home() -> Html<String> {

    let html = fs::read_to_string(view("root.html")).unwrap();

    Html(html.to_string())
}