use super::{routers_demo, routers_hello};
use actix_web::Scope;

pub fn root() -> Scope {
    Scope::new("/api").service(v1()).service(v2())
}

fn v1() -> Scope {
    Scope::new("/v1")
        .service(routers_demo::root())
        .service(routers_hello::root())
}

fn v2() -> Scope {
    Scope::new("/v2").service(routers_hello::root_v2())
}
