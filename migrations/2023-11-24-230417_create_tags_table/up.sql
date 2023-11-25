-- Your SQL goes here
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    title character varying(255) NOT NULL,
    subtitle text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    -- CONSTRAINT tag_id PRIMARY KEY (id)
);