use crate::apis::routers;
use crate::config;
use actix_web::{App, HttpServer};

pub async fn run(cfg: &config::ApiServer) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routers::root()))
        .bind((cfg.host.as_ref(), cfg.port))?
        .run()
        .await
}
