#![allow(unused_imports, dead_code)]

use actix_web::{
    cookie::time::{OffsetDateTime, PrimitiveDateTime},
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use sqlx::{
    error::DatabaseError,
    postgres::{PgPool, PgPoolOptions, PgQueryResult, PgRow, Postgres},
    FromRow, Pool, Row,
};
use std::{env, error::Error};

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
pub struct User {
    user_id: i32,
    username: String,
    email: String,
    handle: String,
    password: String,
    created_at: String,
    update_at: String,
}

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    created_at: String,
    updated_at: String,
    user_id: i32,
}

pub enum DatabaseActionError<T> {
    ReadError(Option<T>),
    WriteError(Option<T>),
    ExternalError(Option<T>),
}

pub async fn index() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn fetch_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    return sqlx::query(r#"SELECT * FROM users"#)
        .map(|user: PgRow| User {
            user_id: user.get(0),
            username: user.get(1),
            email: user.get(2),
            handle: user.get(3),
            password: user.get(4),
            created_at: user.get(5),
            update_at: user.get(6),
        })
        .fetch_all(pool)
        .await;
}
