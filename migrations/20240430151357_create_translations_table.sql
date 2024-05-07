-- Add migration script here
CREATE TABLE translations (
    id SERIAL PRIMARY KEY,
    base_text text NOT NULL,
    target_text text NOT NULL,
    base_language_id INTEGER,
    target_language_id INTEGER,
    script_id INTEGER,
    FOREIGN KEY (base_language_id) REFERENCES languages (id),
    FOREIGN KEY (target_language_id) REFERENCES languages (id),
    FOREIGN KEY (script_id) REFERENCES scripts (id)
);