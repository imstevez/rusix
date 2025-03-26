use crate::config::Config;
use rusix::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let cf = Config::from_yaml().await?;
    let ds = state::State::new(cf).await?;
    api::run_server(ds).await
}
