use actix_web::{HttpResponse, Responder, get};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/1")]
async fn hello_1() -> impl Responder {
    HttpResponse::Ok().body("Hello 1!")
}

#[get("/2")]
async fn hello_2() -> impl Responder {
    HttpResponse::Ok().body("Hello 2!")
}

#[get("")]
async fn hello_v2() -> impl Responder {
    HttpResponse::Ok().body("Hello v2!")
}

#[get("/1")]
async fn hello_1_v2() -> impl Responder {
    HttpResponse::Ok().body("Hello 1 v2!")
}

#[get("/2")]
async fn hello_2_v2() -> impl Responder {
    HttpResponse::Ok().body("Hello 2 v2!")
}
