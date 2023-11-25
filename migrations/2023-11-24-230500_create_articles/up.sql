-- Your SQL goes here
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title character varying(255) NOT NULL,
    subtitle text NOT NULL,
    content text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    --  FOREIGN KEY (id) REFERENCES tags(id) ON DELETE CASCADE
);
