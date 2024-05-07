-- Add migration script here
create TABLE user_languages (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    base_language_id INTEGER,
    target_language_id INTEGER,
    FOREIGN KEY (user_id)  REFERENCES users (id),
    FOREIGN KEY (base_language_id) REFERENCES languages (id),
    FOREIGN KEY (target_language_id) REFERENCES languages (id)
);