use diesel::sql_types::Date;

pub struct Questions {
    question: String,
    option1: String,
    option2: String,
    option3: String,
    option4: String,
    option5: String,
    option6: String,
    answer_index: i16,
    answer_explain: String,
    created_at: Date,
    updated_at: Date,
}
