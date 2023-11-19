
diesel::table! {
    tags (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Varchar,
        body -> Text,
        published -> Bool,
    }
}