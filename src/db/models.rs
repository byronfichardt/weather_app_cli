use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::notes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::notes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewNote {
    pub title: String,
    pub body: String,
    pub published: bool,
}