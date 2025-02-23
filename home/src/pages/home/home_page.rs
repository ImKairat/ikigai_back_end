use actix_web::dev::HttpServiceFactory;
use actix_files::Files;

pub fn get_home_page() -> impl HttpServiceFactory {
    Files::new("/", "./src/pages/home/static/").index_file("index.html")
}
