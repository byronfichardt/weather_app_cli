// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
