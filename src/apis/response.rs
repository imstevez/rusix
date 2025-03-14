use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::Serialize;

pub const CODE_OK: &str = "OK";
pub const CODE_PARAM_ERROR: &str = "PARAM_ERROR";
pub const CODE_INTERNAL_ERROR: &str = "INTERNAL_ERROR";

#[derive(Serialize)]
pub struct Response<T> {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: CODE_OK,
            message: String::from("ok"),
            data,
        }
    }

    pub fn request_params_error<E: ToString>(err: E) -> Self {
        Self {
            code: CODE_PARAM_ERROR,
            message: err.to_string(),
            data: None,
        }
    }

    pub fn internal_server_error<E: ToString>(err: E) -> Self {
        Self {
            code: CODE_INTERNAL_ERROR,
            message: err.to_string(),
            data: None,
        }
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
