mod models;
mod config;
mod handlers;
mod db;
use actix_web::{HttpServer, App, web, http, Result, get, HttpResponse, Error};
use std::io;
use actix_cors::Cors;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;
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

// // #[warn(non_snake_case)]
// async fn status() -> impl Responder {
//     web::HttpResponse::Ok()
//         .json(Status { status: "UP".to_string() })
// }


#[get("/users/{user_id}/{friend}")]
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    //web::Path((user_id, friend)): web::Path<(u32, String)>
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[warn(non_snake_case)]
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("项目启动了{:?}",config);
    std::fs::create_dir_all("./tmp").unwrap();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:63343")
            .allowed_origin("http://localhost:8080")
            // allow_any_origin 全部都跨域  多个跨域设置多个
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().ends_with(b".rust-lang.org")
            // })
            .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .data(pool.clone())
            .wrap(cors)
            .service(index)
            .route("/", web::get().to(status))
            .route("/", web::post().to(save_file))
            .route("/todos{_:/?}", web::get().to(todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/get/{list_id}/items{_:/?}", web::get().to(get_todo_items))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}