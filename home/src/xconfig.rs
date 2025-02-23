
#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Option<Config> {
    Some(Config{
        host: "localhost".to_string(),
        port: 8080,
    })
}