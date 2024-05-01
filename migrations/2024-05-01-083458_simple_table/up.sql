-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    first_name non_empty_text NOT NULL,
    middle_name non_empty_text,
    last_name non_empty_text NOT NULL
);