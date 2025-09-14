#![allow(dead_code, unused_imports)]
use actix_web::Responder;
use actix_files::NamedFile;


pub async fn chat_page() -> impl Responder {
    NamedFile::open("front/chat/chat.html").unwrap()
}
