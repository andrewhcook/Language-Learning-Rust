use bcrypt::*;
use axum_macros::debug_handler;
use axum::http::{StatusCode};
use jsonwebtoken::{encode,decode,  Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool};
use axum::{Extension, Json};
use crate::errors::CustomError;
use std::time::{SystemTime, UNIX_EPOCH};
use Language_Learning_Backend::*;


#[derive(Deserialize,Serialize, Debug)]
pub struct Claims {
    user: User,
    exp: u64
}

// get self
pub async fn get_self(Json(token) : Json<Token>, 
Extension(_pool): Extension<PgPool>) -> Result<(StatusCode,Json<User>), CustomError> {
    match verify_token(token.clone()) {
        Ok(user) => {return Ok((StatusCode::OK, Json(user)))},
        Err(_e) => {return Err(CustomError::InvalidCrendentials)}
    }
}

// login (authorize user)
pub async fn login(Json(login_info): Json<LoginCredentials>,
Extension(pool): Extension<PgPool>) -> Result<(StatusCode, Json<Token>), CustomError> {
    let sql = "SELECT * FROM users where username=($1);".to_string();
    let user: User = sqlx::query_as::<_,User>(&sql)
    .bind(login_info.username)
    .fetch_one(&pool)
    .await.map_err( |x| {
        println!("{}",x);
        CustomError::InternalServerError
    })?;
    match verify(login_info.password, &user.salted_hash) {
        Ok(true) => {
            let key = std::env::var("SECRET_KEY").map_err(|x| {CustomError::InternalServerError})?;
            let encode_key = EncodingKey::from_base64_secret(&key).map_err(|x| {
                println!("{}",x);
                CustomError::InternalServerError})?;
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            let exp = now.as_secs() + 8 * 24 * 60 * 60; // 8 days in seconds
            let claims = Claims {user, exp};
            let encode_string = encode(&Header::default(), &claims, &encode_key).map_err(|x| {CustomError::InternalServerError})?;
            let token = Token(encode_string);
            return Ok((StatusCode::OK,Json(token)))},
        Ok(false) => {return Err(CustomError::InvalidCrendentials)},
        _ => {return Err(CustomError::InternalServerError)}
    };
}


// verify token (authenticate user)
pub fn verify_token(Token(str_key): Token) -> Result<User, CustomError> {
    let key = std::env::var("SECRET_KEY").map_err(|x| {CustomError::InternalServerError})?;
    let decode_key = DecodingKey::from_base64_secret(&key).map_err(|x| {
        println!("{}",x);
        CustomError::InternalServerError})?;
    let user_result =  decode::<Claims>(&str_key, &decode_key, &Validation::new(Algorithm::HS256)).map_err(|x| {println!("{}",x);CustomError::InternalServerError})?;
    return Ok(user_result.claims.user)
}
// get translation

// register
#[debug_handler]
pub async fn register( Extension(pool): Extension<PgPool>, Json(registration_info): Json<RegistrationInfo>) -> Result<(StatusCode, Json<Token>), CustomError> {
    let salted_hash = hash(registration_info.password, DEFAULT_COST).map_err(|x| {CustomError::InternalServerError})?;
    let sql = "INSERT into users (username, salted_hash, email, bio, contact_info, premium) values ($1,$2,$3,$4,$5,$6) RETURNING *;";
    let user = sqlx::query_as::<_,User>(&sql)
    .bind(&registration_info.username)
    .bind(&salted_hash)
    .bind(&registration_info.email)
    .bind(&registration_info.bio)
    .bind(&registration_info.contact_info)
    .bind(&registration_info.premium)
    .fetch_one(&pool)
    .await.map_err(|x| {
        println!("{}", x);
        CustomError::InternalServerError})?;

    let key = std::env::var("SECRET_KEY").map_err(|x| {
        CustomError::InternalServerError})?;
    let encode_key = EncodingKey::from_base64_secret(&key).map_err(|x| {
        println!("{}",x);
        CustomError::InternalServerError})?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let exp = now.as_secs() + 8 * 24 * 60 * 60; // 8 days in seconds
    let claims = Claims {user, exp};
    let encode_string = encode(&Header::default(), &claims, &encode_key).map_err(|x| {CustomError::InternalServerError})?;
    let token = Token(encode_string);
    return Ok((StatusCode::CREATED, Json(token)))
}

//user count is for debugging purposes:

pub async fn user_count(Extension(pool): Extension<PgPool>) -> Result<Json<i32>, CustomError> {
    let sql = "SELECT * from users;".to_string();
    let user_count = sqlx::query(&sql).fetch_all(&pool).await.unwrap().iter().filter(|_|{true} ).count();
    println!("{}", user_count.clone());
    return Ok(Json(user_count as i32))
}

// update user question

// get next translation in sequence

// fetch a script by id
pub async fn fetch_script_by_id(Extension(pool): Extension<PgPool>, Json(script_id): Json<i32>) -> Result<Json<Script>, CustomError> {
    let sql = "SELECT * FROM scripts WHERE id =($1);".to_string();
    let script = sqlx::query_as::<_,Script>(&sql).bind(script_id).fetch_one(&pool).await.map_err(|x| {CustomError::InternalServerError})?;
    Ok(Json(script))
}
// see all scripts in a language

pub async fn fetch_scripts_for_language(Extension(pool): Extension<PgPool>, Json(langauge_id): Json<i32>) -> Result<Json<Vec<Script>>, CustomError> {
    let sql = "SELECT * from scripts where base_language_id = ($1);".to_string();
    let scripts = sqlx::query_as::<_,Script>(&sql).bind(&langauge_id).fetch_all(&pool).await.map_err(|x| {CustomError::InternalServerError})?;
    return Ok(Json(scripts))
}

pub async fn get_translation_count(Extension(pool): Extension<PgPool>) -> Result<Json<i32>, CustomError> {
    let sql = "SELECT * from translations;".to_string();
    let count = sqlx::query(&sql).fetch_all(&pool).await.unwrap().iter().filter(|_|{true} ).count();
    println!("{}", count);
    Ok(Json(count as i32))
}

// get records of all languages

pub async fn fetch_all_languages(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Language>>, CustomError> {
    let sql = "SELECT * FROM languages;".to_string();
    let languages = sqlx::query_as::<_,Language>(&sql).fetch_all(&pool).await.map_err(|x| CustomError::InternalServerError)?;
    println!("{:?}", languages.clone());
    return Ok(Json(languages))
}

pub async fn count_all_languages(Extension(pool): Extension<PgPool>) -> Result<Json<i32>, CustomError> {
    let sql = "SELECT * FROM languages;".to_string();
    let language_count = sqlx::query_as::<_,Language>(&sql).fetch_all(&pool).await.map_err(|x| CustomError::InternalServerError)?.into_iter().count();
    return Ok(Json(language_count as i32))
}

// get all langauges for a user
pub async fn fetch_all_languages_for_user(Extension(pool): Extension<PgPool>, Json(token): Json<Token>) -> Result<Json<Vec<UserLanguage>>, CustomError> {
    let user = verify_token(token)?;
    let sql = "SELECT * FROM user_languages  
    JOIN languages on user_languages.language_id=languages.id
    WHERE user_id = ($1);";
    let languages = sqlx::query_as::<_,UserLanguage>(&sql).bind(user.id).fetch_all(&pool).await.map_err(|x| {CustomError::InternalServerError})?;
    Ok(Json(languages))
}



// get user_by_id

//get language_by_id

pub async fn fetch_language_by_id(Extension(pool): Extension<PgPool>, Json(language_id): Json<i32>) -> Result<Json<Language>, CustomError> {
     let sql = "SELECT * FROM languages WHERE id =($1);".to_string();
     let language = sqlx::query_as::<_,Language>(&sql).bind(language_id).fetch_one(&pool).await.map_err(|x| {CustomError::InternalServerError})?;
     Ok(Json(language))
}

#[debug_handler]
pub async fn add_new_user_language( Extension(pool): Extension<PgPool>, Json((new_user_language_info, token)): Json<(NewUserLanguageFormData, Token)>) -> Result<StatusCode, CustomError> {

    let user =  verify_token(token).map_err(|x|CustomError::InvalidCrendentials)?;
    let sql = "INSERT INTO user_languages (user_id, base_language_id, target_language_id) values ($1, $2, $3) RETURNING *;".to_string();
    let _ = sqlx::query(&sql).bind(user.id).bind(new_user_language_info.base_language_id).bind(new_user_language_info.target_language_id).execute(&pool).await.map_err(|x| CustomError::InternalServerError)?;
    Ok(StatusCode::CREATED)
}

#[debug_handler]
pub async fn add_new_language( Extension(pool): Extension<PgPool>, Json(new_language_info): Json<NewLanguageData>) -> Result<(StatusCode, Json<Language>), CustomError> {
    let sql = "INSERT INTO languages (title, shortcode) VALUES ($1, $2);".to_string();
    let language = sqlx::query_as::<_,Language>(&sql).bind(new_language_info.title).bind(new_language_info.shortcode).fetch_one(&pool).await.map_err(|x| 
        {println!("{}",x);
    CustomError::InternalServerError})?;
    Ok((StatusCode::CREATED, Json(language)))
}

#[debug_handler]
pub async fn add_new_script( Extension(pool): Extension<PgPool>, Json(new_script_info): Json<NewScriptData>) -> Result<(StatusCode, Json<Script>), CustomError> {
    let sql = "INSERT INTO scripts (title, description_, author, publishing_year, complexity, base_language_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;".to_string();
    let script = sqlx::query_as::<_,Script>(&sql).bind(new_script_info.title).bind(new_script_info.description_).bind(new_script_info.author).bind(new_script_info.publishing_year).bind(new_script_info.complexity).bind(new_script_info.base_language_id).fetch_one(&pool).await.map_err(|x|{ println!("{}", x); CustomError::InternalServerError})?;
    Ok((StatusCode::CREATED, Json(script)))
}

#[debug_handler]
pub async fn add_new_translation( Extension(pool): Extension<PgPool>, Json(new_translation_info, ): Json<NewTranslationData,>) -> Result<(StatusCode,Json<Translation>), CustomError> {
    let sql = "INSERT INTO translations (base_text, target_text, base_language_id, target_language_id, script_id) VALUES ($1, $2, $3, $4, $5) RETURNING *;".to_string();
    let translation = sqlx::query_as::<_,Translation>(&sql).bind(new_translation_info.base_text).bind(new_translation_info.target_text).bind(new_translation_info.base_language_id).bind(new_translation_info.target_language_id).bind(new_translation_info.script_id).fetch_one(&pool).await.map_err(|x| 
        {println!("{}",x);
            CustomError::InternalServerError})?;
    Ok((StatusCode::CREATED, Json(translation)))
}