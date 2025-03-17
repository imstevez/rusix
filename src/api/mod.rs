mod handlers;
mod middlewares;
mod response;
mod routers;

use crate::datasource::Datasource;
use actix_web::{App, HttpServer};

pub async fn run_server(ds: Datasource) -> std::io::Result<()> {
    let cf = ds.cf.api_server.clone();
    HttpServer::new(move || App::new().service(routers::api(ds.clone())))
        .bind((cf.host, cf.port))?
        .run()
        .await
}
