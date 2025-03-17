use crate::config::Config;
use log;
use rusix::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let cf = Config::from_yaml().await?;
    let ds = datasource::Datasource::new(cf)?;
    api::run_server(ds).await
}
