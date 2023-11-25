-- Your SQL goes here
CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    question TEXT NOT NULL,
    option1 TEXT NOT NULL,
    option2 TEXT NOT NULL,
    option3 TEXT NOT NULL,
    option4 TEXT NOT NULL,
    option5 TEXT NOT NULL,
    option6 TEXT NOT NULL,
    answer_index SMALLINT NOT NULL,
    answer_explain TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    --  FOREIGN KEY (id) REFERENCES tags(id) ON DELETE CASCADE
);
