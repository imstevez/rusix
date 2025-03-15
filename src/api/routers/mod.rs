use actix_web::Scope;

mod demo;
mod hello;

pub fn routers() -> Scope {
    Scope::new("/api").service(v1()).service(v2())
}

fn v1() -> Scope {
    Scope::new("/v1")
        .service(demo::routers())
        .service(hello::routers())
}

fn v2() -> Scope {
    Scope::new("/v2").service(hello::routers_v2())
}
