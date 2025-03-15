use crate::api::handlers::hello::*;
use actix_web::Scope;

pub fn routers() -> Scope {
    Scope::new("/hello")
        .service(hello)
        .service(hello_1)
        .service(hello_2)
}

pub fn routers_v2() -> Scope {
    Scope::new("/hello")
        .service(hello_v2)
        .service(hello_1_v2)
        .service(hello_2_v2)
}
