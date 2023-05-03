// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        description -> Varchar,
        price -> Int4,
        name -> Varchar,
        brand -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
