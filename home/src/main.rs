use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tokio::fs;

#[get("/")]
async fn index() -> impl Responder {
    match fs::read_to_string("./static/index.html").await {
        Ok(contents) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(contents),
        Err(_) => HttpResponse::InternalServerError()
            .body("Error loading index.html"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (host, port) = ("127.0.0.1", 8080);
    println!("\nServer running at http://{}:{}\n", host, port);

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind((host, port))?
    .run()
    .await
}
