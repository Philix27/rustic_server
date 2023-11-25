// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        subtitle -> Text,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    question_group (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        subtitle -> Text,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        question -> Text,
        option1 -> Text,
        option2 -> Text,
        option3 -> Text,
        option4 -> Text,
        option5 -> Text,
        option6 -> Text,
        answer_index -> Int2,
        answer_explain -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        subtitle -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    question_group,
    questions,
    tags,
);
