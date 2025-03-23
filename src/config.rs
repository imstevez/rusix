use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_server: ApiServer,
    pub rw_db: Database,
    pub redis: Redis,
}

impl Config {
    const CONFIG_FILE_PATH_ENV: &'static str = "CONFIG_FILE";
    const CONFIG_FILE_PATH_DEFAULT: &'static str = "./config.yaml";

    fn file_path() -> io::Result<String> {
        match env::var(Self::CONFIG_FILE_PATH_ENV) {
            Ok(path) => Ok(path),
            Err(env::VarError::NotPresent) => Ok(Self::CONFIG_FILE_PATH_DEFAULT.to_string()),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub async fn from_yaml() -> io::Result<Self> {
        let path = Config::file_path()?;
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let conf =
            serde_yaml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(conf)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiServer {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub database: u16,
}
