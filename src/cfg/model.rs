use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DataBaseConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DataBaseConfig {
    pub postgres: PostgresConfig,
    pub mongo: MongoConfig,
}

#[derive(Deserialize, Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub table: String,
}

#[derive(Deserialize, Debug)]
pub struct MongoConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}