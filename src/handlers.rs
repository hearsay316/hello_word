use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, http, Result, get, HttpResponse, Error};

// #[warn(non_snake_case)]
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string() })
}