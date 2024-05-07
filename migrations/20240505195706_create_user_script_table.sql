-- Add migration script here
create TABLE user_scripts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    script_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (script_id) REFERENCES scripts (id)
);