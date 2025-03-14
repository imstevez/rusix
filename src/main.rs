use tokio::io;
use crate::config::Configs;
mod config;

#[tokio::main]
async fn main() -> io::Result<()> {
    let configs = Configs::from_yaml().await?;
    println!("Loaded configs: {:?}", configs);
    Ok(())
}
