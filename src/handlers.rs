use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, http, Result, get, HttpResponse, Error};
use deadpool_postgres::{Pool, Client};
use crate::db;
// #[warn(non_snake_case)]
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string() })
}

pub async fn todos(
    db_pool: web::Data<Pool>
) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_todo_s(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}