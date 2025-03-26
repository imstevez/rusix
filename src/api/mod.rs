mod handlers;
mod middlewares;
mod response;
mod routers;

use crate::state::State;
use actix_web::{App, HttpServer};

pub async fn run_server(state: State) -> std::io::Result<()> {
    let cf = state.cfg.api_server.clone();
    HttpServer::new(move || App::new().service(routers::api(state.clone())))
        .bind((cf.host, cf.port))?
        .run()
        .await
}
