use crate::apis::routers;
use crate::configs::Configs;
use actix_web::{App, HttpServer};

pub async fn run(configs: Configs) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routers::root()))
        .bind((configs.api_server.host, configs.api_server.port))?
        .run()
        .await
}
