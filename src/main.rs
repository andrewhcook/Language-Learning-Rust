use api::route_functions::{self, get_translation_count};
use axum::{
    routing::{get, post}, Router, extract::{Extension},
};
use sqlx::postgres::PgPoolOptions;
mod api;
pub mod errors;
use anyhow::{Context, Ok};
use dotenv;
use http::{Method};
use tower_http::cors::{CorsLayer, Any};


#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    dotenv::dotenv().ok();
    let database_url =std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
    .max_connections(50)
    .connect(&database_url)
    .await.context("unable to connect to database")?;


    let app = Router::new()
    .route("/hello", get(|| async { "Hello, World!" }))
    .route("/register", post( api::route_functions::register))
    .route("/user_count", get(api::route_functions::user_count)) // not in production
    .route("/language_count", get(api::route_functions::count_all_languages)) // not in production
    .route("/translation_count", get(api::route_functions::get_translation_count))
    .route("/all_languages", get(api::route_functions::fetch_all_languages))
    .route("/add_language", post(api::route_functions::add_new_language))
    .route("/add_script", post(api::route_functions::add_new_script))
    .route("/add_translation", post(api::route_functions::add_new_translation))
    .layer(
        CorsLayer::new()
        .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any)
    )
    .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}