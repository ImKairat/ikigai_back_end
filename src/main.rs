use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const IP_ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    HttpResponse::Ok().body(body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Manual Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\nStarting server on http://{}:{}\n", IP_ADDRESS, PORT);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service()
            .route("/manual", web::get().to(manual_hello))
    })
        .bind(format!("{IP_ADDRESS}:{PORT}"))?
        .run()
        .await
}