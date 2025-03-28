use crate::api::middlewares::*;
use crate::state::State;
use actix_web::Scope;
use actix_web::dev::HttpServiceFactory;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::web::Data;

mod posts;

pub fn api(ds: State) -> impl HttpServiceFactory {
    Scope::new("/api")
        .wrap(ErrorHandlers::new().default_handler(handle_err))
        .wrap(Logger::default())
        .app_data(Data::new(ds.clone()))
        .service(v1())
        .service(v2())
}

fn v1() -> impl HttpServiceFactory {
    Scope::new("/v1").service(posts::v1())
}

fn v2() -> impl HttpServiceFactory {
    Scope::new("/v2")
}
