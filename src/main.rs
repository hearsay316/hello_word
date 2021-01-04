
// extern crate actix_web;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
#[warn(non_snake_case)]
async fn status()->impl Responder{
    "{\"status\": \"UP\"}"
}
#[warn(non_snake_case)]
#[actix_web::main]
async fn main() ->io::Result<()>{
    println!("项目启动了在 127.0.0.1:8080端口3");
    HttpServer::new(|| {
        App::new().route("/",web::get().to(status))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}