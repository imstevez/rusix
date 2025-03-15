mod handlers;
mod response;
mod routers;

use crate::api::routers::*;
use crate::config::Config;
use actix_web::{App, HttpServer};

pub async fn run_server(configs: Config) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routers()))
        .bind((configs.api_server.host, configs.api_server.port))?
        .run()
        .await
}
