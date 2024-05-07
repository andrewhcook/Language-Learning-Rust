-- Add migration script here
CREATE TABLE user_questions (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    seen_count INTEGER,
    attempts INTEGER,
    success INTEGER,
    user_id INTEGER,
    translation_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (translation_id) REFERENCES translations (id)
    );