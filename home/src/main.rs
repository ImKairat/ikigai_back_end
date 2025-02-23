mod config;
mod pages;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use config::get_config;
use pages::home::get_home_page;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = get_config().unwrap();
    println!("\nServer run on http://{}:{}\n", cfg.host, cfg.port);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                get_home_page()
            )
    })
    .bind((cfg.host, cfg.port))?
    .run()
    .await
}
