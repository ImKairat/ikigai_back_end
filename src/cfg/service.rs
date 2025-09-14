use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::cfg::model::{Config, DataBaseConfig, MongoConfig, PostgresConfig, ServerConfig};

const CONFIG_FILE_PATH: &str = "./src/config.yaml";

pub async fn read_yaml() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(CONFIG_FILE_PATH).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub async fn get_server_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let config = read_yaml().await?;
    Ok(config.server)
}

pub async fn get_mongo_config() -> Result<MongoConfig, Box<dyn std::error::Error>> {
    let config = read_yaml().await?;
    Ok(config.database.mongo)
}

pub async fn get_postgres_config() -> Result<PostgresConfig, Box<dyn std::error::Error>> {
    let config = read_yaml().await?;
    Ok(config.database.postgres)
}