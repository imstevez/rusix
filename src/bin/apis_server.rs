use crate::configs::Configs;
use rusix::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    apis::server::run(Configs::from_yaml().await?).await
}
