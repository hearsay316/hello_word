mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, http, Result, get, HttpResponse,Error};
use std::io;
use actix_cors::Cors;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
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
    std::fs::create_dir_all("./tmp").unwrap();
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
            .route("/", web::get().to(status))
            .route("/",web::post().to(save_file))
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}