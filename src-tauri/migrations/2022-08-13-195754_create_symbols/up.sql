-- Your SQL goes here

CREATE TABLE symbols (
    id SERIAL PRIMARY KEY,
    unicode_number VARCHAR,
    identifier VARCHAR NOT NULL,
    is_composite BOOLEAN NOT NULL
)