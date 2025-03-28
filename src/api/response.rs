use crate::api::response::Code::{InternalError, NotFound, RequestError};
use actix_web::http::StatusCode;
use actix_web::{
    HttpRequest, HttpResponse, Responder, ResponseError, body::BoxBody, http::header::ContentType,
};
use deadpool::managed::PoolError;
use derive_more::Display;
use serde::Serialize;
use serde_json::json;
use validator::{ValidationError, ValidationErrors};

#[derive(Serialize, Debug, Display)]
pub enum Code {
    #[display("OK")]
    #[serde(rename = "OK")]
    Ok,
    #[display("NOT_FOUND")]
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[display("REQUEST_ERROR")]
    #[serde(rename = "REQUEST_ERROR")]
    RequestError,
    #[display("INTERNAL_ERROR")]
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
}

pub struct Empty;

impl Responder for Empty {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = json!({"code": Code::Ok, "message": "ok"});
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body.to_string())
    }
}

#[derive(Serialize, Debug, Display)]
pub struct Data<T: Serialize>(pub T);

impl<T: Serialize> Responder for Data<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = json!({"code": Code::Ok, "message": "ok", "data": self.0});
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body.to_string())
    }
}

#[derive(Serialize, Debug, Display)]
#[display("Err<{}>: {}", _0, _1)]
pub struct Error(Code, String);

impl Error {
    pub fn not_found() -> Self {
        Self(NotFound, "Not Found".to_string())
    }

    pub fn request_error(message: String) -> Self {
        Self(RequestError, message)
    }

    pub fn internal_error(message: String) -> Self {
        Self(InternalError, message)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = json!({"code": self.0, "message": self.1});
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(body.to_string())
    }
}

impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        Error(RequestError, err.to_string())
    }
}

impl From<ValidationErrors> for Error {
    fn from(err: ValidationErrors) -> Self {
        Error(RequestError, err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error(InternalError, err.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error(InternalError, err.to_string())
    }
}

impl From<PoolError<diesel_async::pooled_connection::PoolError>> for Error {
    fn from(err: PoolError<diesel_async::pooled_connection::PoolError>) -> Self {
        Error(InternalError, err.to_string())
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error(InternalError, err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error(InternalError, err.to_string())
    }
}
