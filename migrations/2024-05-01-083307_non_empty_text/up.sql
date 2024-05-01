-- This migration create a custom data type called non_empty_text where the text field cannot be empty
-- This should not use DOMAIN nor CHECK constraint because Diesel detects them as "normal" text fields
-- We then need to create a custom type, custom input and output functions

CREATE DOMAIN non_empty_text_in AS TEXT
    CHECK (value <> '');

CREATE type non_empty_text AS (
    value non_empty_text_in
);