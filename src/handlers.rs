use crate::models::{Status, TodoItem};
use actix_web::{HttpServer, App, web, Responder, http, Result, get, HttpResponse, Error};
use deadpool_postgres::{Pool, Client};
use crate::db;
use std::io;

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

pub async fn get_todo_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    println!("{:?}",path);
    let result = db::get_item(&client, path.into_inner().0 ).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

