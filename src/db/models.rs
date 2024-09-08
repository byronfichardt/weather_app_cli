use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::notes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}