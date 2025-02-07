#![allow(dead_code, unused_imports)]
use actix_web::Responder;
use actix_files::NamedFile;


pub async fn home_page() -> impl Responder {
    NamedFile::open("front/home/index.html").unwrap()
}
