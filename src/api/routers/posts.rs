use crate::api::handlers::posts::*;
use actix_web::Scope;

pub fn v1() -> Scope {
    Scope::new("/posts")
        .service(list_posts)
        .service(create_post)
}
