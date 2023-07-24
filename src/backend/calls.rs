#![allow(unused_imports, dead_code)]

use super::models::User;

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

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;

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
