use crate::config::Config;
use rusix::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    api::run_server(Config::from_yaml().await?).await
}
