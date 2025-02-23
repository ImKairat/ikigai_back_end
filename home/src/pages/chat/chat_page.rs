use actix_web::dev::HttpServiceFactory;
use actix_files::Files;

#[allow(dead_code)]
pub fn get_chat_page() -> impl HttpServiceFactory {
    Files::new("/", "./src/pages/chat/static/").index_file("chat.html")
}
