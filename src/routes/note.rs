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

pub async fn get_notes( 
    Extension(pool): Extension<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<Json<Vec<NoteStruct>>, StatusCode> {
    use crate::db::schema::notes::dsl::*;

    let conn = &mut pool.get().unwrap();

    let note_results = notes
        .filter(published.eq(true))
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
) -> Result<(), StatusCode> {
    use crate::db::schema::notes::dsl::*;

    let conn = &mut pool.get().unwrap();

    let new_note = Note {
        id: 1234,
        title: "this is a note".to_string(),
        body: "this is a note body".to_string(),
        published: true
    };

    let rows_inserted = diesel::insert_into(notes)
        .values(&new_note)
        .execute(conn);

    assert_eq!(Ok(1), rows_inserted);

    Ok(())
}
