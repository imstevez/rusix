use rusix::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // load configs
    let configs = config::Configs::from_yaml().await?;
    println!("Loaded configs: {:?}", configs);

    // run apis server
    apis::server::run(&configs.api_server).await
}
