use crate::apis::handlers_demo;
use actix_web::Scope;

pub fn root() -> Scope {
    Scope::new("/demo")
        .service(handlers_demo::echo)
        .service(handlers_demo::ok)
        .service(handlers_demo::error)
}
