use diesel::MysqlConnection;
use axum::{Extension, Json};
use bson::{Uuid, doc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::models::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestInput {
    id: String,
    body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryParams {
    note: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseValue {
    status_code: String,
    id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NoteStruct {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NotePostRequest {
    title: String,
    body: String,
    published: bool,
}

pub async fn get_notes( 
    Extension(pool): Extension<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<Json<Vec<NoteStruct>>, StatusCode> {
    use crate::db::schema::notes::dsl::*;

    let conn = &mut pool.get().unwrap();

    let note_results = notes
        .limit(5)
        .select(Note::as_select())
        .load(conn)
        .expect("Error loading notes");

    let mut results = Vec::new();
    for note in note_results {
        results.push(NoteStruct {
            id: note.id,
            title: note.title,
            body: note.body,
            published: note.published
        })
    }

    Ok(Json(results))
}


pub async fn create_note( 
    Extension(pool): Extension<Pool<ConnectionManager<MysqlConnection>>>,
    Json(payload): Json<NotePostRequest>,
) -> Result<Json<NoteStruct>, StatusCode> {
    use crate::db::schema::notes::dsl::*;

    let conn = &mut pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_note = NewNote {
        title: payload.title.clone(),
        body: payload.body,
        published: payload.published
    };

    let inserted_rows = diesel::insert_into(notes)
        .values(&new_note)
        .execute(conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let result: Note = notes
        .filter(title.eq(&payload.title))
        .order(id.desc())  // Get the latest inserted note
        .first(conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let inserted_note = NoteStruct {
        id: result.id,
        title: result.title,
        body: result.body,
        published: result.published
    };

    Ok(Json(inserted_note))
}
