mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, http, Result, get};
use std::io;
use actix_cors::Cors;
// #[warn(non_snake_case)]
async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string() })
}


#[get("/users/{user_id}/{friend}")]
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    //web::Path((user_id, friend)): web::Path<(u32, String)>
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[warn(non_snake_case)]
#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("项目启动了在 127.0.0.1:8080");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:63342")
            .allowed_origin("http://localhost:8080")
            // allow_any_origin 全部都跨域  多个跨域设置多个
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().ends_with(b".rust-lang.org")
            // })
            .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new().wrap(cors)
            .service(index)
            .route("/", web::get().to(status),
            )
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}