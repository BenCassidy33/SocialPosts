#![allow(unused_imports)]

mod calls;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("localhost", 8080))?
        .run()
        .await
}

#[get("/get/users")]
async fn index() -> impl Responder {
    let pool = calls::index().await.ok();

    return match calls::fetch_users(&pool.unwrap()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    };
}
