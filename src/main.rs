#![allow(dead_code, unused_imports, unused_variables)]
mod routes;
mod cfg;

use std::error::Error;
use routes::{home::home_page, chat::chat_page};
use cfg::{get_server_config, ServerConfig};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_files::Files;

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    HttpResponse::Ok().body(body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Manual Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = get_server_config().await.unwrap();
    let (host, port) = (&server.host, server.port);
    
    println!("\nStarting server on http://{}:{}\n", host, port);
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home_page))
            .route("/chat", web::get().to(chat_page))
            .service(Files::new("/static", "front/home/static").show_files_listing())
            .service(Files::new("/static", "front/chat/static").show_files_listing())
    })
        .bind(format!("{host}:{port}"))?
        .run()
        .await
}
