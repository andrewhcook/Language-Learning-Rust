-- Add migration script here
CREATE TABLE scripts (
    id SERIAL PRIMARY KEY,
    title varchar(255) UNIQUE NOT NULL,
    description_ text NOT NULL,
    author varchar(255) NOT NULL,
    publishing_year INTEGER,
    complexity INTEGER,
    base_language_id INTEGER NOT NULL,
    FOREIGN KEY (base_language_id) REFERENCES languages (id)
);