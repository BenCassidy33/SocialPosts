#![allow(unused_imports)]

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;

pub mod backend;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("localhost", 8080))?
        .run()
        .await
}

#[get("/get/users")]
async fn index() -> impl Responder {
    let pool = backend::calls::create_pool().await;

    return match pool {
        Ok(_) => match backend::calls::fetch_users(&pool.unwrap()).await {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(e) => HttpResponse::InternalServerError()
                .body(format!("Failed to fetch user data {}", e.to_string())),
        },

        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to database"),
    };
}
