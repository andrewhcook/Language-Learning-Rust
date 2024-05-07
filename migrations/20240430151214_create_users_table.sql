-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username varchar(50) NOT NULL UNIQUE,
    salted_hash varchar(255) NOT NULL,
    email varchar(255) NOT NULL UNIQUE,
    bio TEXT,
    contact_info TEXT,
    premium tier_level NOT NULL DEFAULT 'Free'
);