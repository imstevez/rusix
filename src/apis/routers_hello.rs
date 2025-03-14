use crate::apis::handlers_hello;
use actix_web::Scope;

pub fn root() -> Scope {
    Scope::new("/hello")
        .service(handlers_hello::hello)
        .service(handlers_hello::hello_1)
        .service(handlers_hello::hello_2)
}

pub fn root_v2() -> Scope {
    Scope::new("/hello")
        .service(handlers_hello::hello_v2)
        .service(handlers_hello::hello_1_v2)
        .service(handlers_hello::hello_2_v2)
}
