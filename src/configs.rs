use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configs {
    pub api_server: ApiServer,
}

impl Configs {
    const CONFIGS_FILE_PATH_ENV: &'static str = "CONFIGS_FILE_PATH";
    const CONFIGS_FILE_PATH_DEFAULT: &'static str = "./configs.yaml";

    fn file_path() -> io::Result<String> {
        match env::var(Self::CONFIGS_FILE_PATH_ENV) {
            Ok(path) => Ok(path),
            Err(env::VarError::NotPresent) => Ok(Self::CONFIGS_FILE_PATH_DEFAULT.to_string()),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub async fn from_yaml() -> io::Result<Self> {
        let path = Configs::file_path()?;
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
