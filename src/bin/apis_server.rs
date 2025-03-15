use crate::configs::Configs;
use rusix::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    api::run_server(Configs::from_yaml().await?).await
}
