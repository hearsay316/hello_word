use crate::models::{Status, CreateTodoList, ResultResponse};
use actix_web::{ web, Responder,  HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::db;
use std::io::ErrorKind::Other;

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
    let result = db::get_items(&client, path.into_inner().0 ).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_todo(&client,json.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
pub async fn create_item(db_pool: web::Data<Pool>, web::Path((username, count)): web::Path<(i32, i32)>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    println!("{}----{}",username, count);
    let result = db::
    check_item(&client, username, count).await;
    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse{success:true}),
        Err(ref e) if e.kind() == Other =>HttpResponse::Ok().json(ResultResponse{success:false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}