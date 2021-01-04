mod models;

// extern crate actix_web;
use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, http};
use std::io;
use actix_cors::Cors;

#[warn(non_snake_case)]
async fn status() -> impl Responder {
   web::HttpResponse::Ok()
       .json(Status{status: "UP".to_string()})
}

#[warn(non_snake_case)]
#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("项目启动了在 127.0.0.1:8080端口3");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8081")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".rust-lang.org")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new().wrap(cors).route("/", web::post().to(status))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}