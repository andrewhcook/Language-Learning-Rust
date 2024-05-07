-- Add migration script here
CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    title varchar(50) NOT NULL UNIQUE,
    shortcode TEXT UNIQUE
);