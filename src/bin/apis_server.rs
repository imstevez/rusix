use rusix::*;
use crate::configs::Configs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // load configs
    let configs = Configs::from_yaml().await?;
    println!("Loaded configs: {:?}", configs);

    // run apis server
    apis::server::run(configs).await
}
