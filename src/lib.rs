use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub salted_hash: String,
    pub email: String,
    pub bio: String,
    pub contact_info: String,
    pub premium: String
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct Token(pub String);

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct RegistrationInfo {
    pub username: String,
    pub password: String,
    pub email: String,
    pub bio: String,
    pub contact_info: String,
    pub premium: String,
}


#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Translation {
    pub id: i32,
    pub base_text: String,
    pub target_text: String,
    pub base_language_id: i32,
    pub target_language_id: i32,
    pub script_id: i32,
}


#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct UserQuestioon {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub seen_count: i32,
    pub attempts: i32,
    pub success: i32,
    pub user_id: i32,
    pub translation_id: i32,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Script {
    pub id: i32,
    pub title: String,
    pub descript: String,
    pub author: String,
    pub publishing_year: i32,
    pub complexity: i32,
    pub base_language_id: i32,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct Language {
    pub id: i32,
    pub title: String,
    pub shortcode: String
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct UserLanguage {
    pub id: i32,
    pub user_id: i32,
    pub base_language_id: i32,
    pub target_language_id: i32
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewUserLanguageFormData {
    pub user_id: i32,
    pub base_language_id: i32,
    pub target_language_id: i32
}


#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewLanguageData {
    pub title: String,
    pub shortcode: String
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewScriptData {
    pub title: String,
    pub description_: String,
    pub author: String,
    pub publishing_year: i32,
    pub complexity: i32,
    pub base_language_id: i32,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewTranslationData {
    pub base_text: String,
    pub target_text: String,
    pub base_language_id: i32,
    pub target_language_id: i32,
    pub script_id: i32,
}