use crate::api::handlers::demo::*;
use actix_web::Scope;

pub fn routers() -> Scope {
    Scope::new("/demo").service(echo).service(ok).service(error)
}
