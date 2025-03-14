use crate::apis::response::Response;
use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Validate, Deserialize)]
struct EchoReq {
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

#[get("")]
async fn echo(req: web::Json<EchoReq>) -> impl Responder {
    match req.validate() {
        Ok(_) => Response::ok(Some(req)),
        Err(err) => Response::request_params_error(err),
    }
}

#[get("/ok")]
async fn ok() -> impl Responder {
    Response::<&str>::ok(Some("mock ok data"))
}

#[get("/error")]
async fn error() -> impl Responder {
    Response::<&str>::internal_server_error("mock internal server error")
}
