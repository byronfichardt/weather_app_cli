use crate::db::schema::items;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub description: String,
    pub price: i32,
    pub name: String,
    pub brand: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub description: &'a str,
    pub price: &'a i32,
    pub name: &'a str,
    pub brand: &'a str,
}

impl NewItem<'_> {
    pub fn create(self: &Self, connection: &mut PgConnection) -> Item {
        diesel::insert_into(items::table)
            .values(self)
            .get_result(connection)
            .expect("Error saving item")
    }
}
