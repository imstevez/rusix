use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::Serialize;

#[derive(Serialize)]
pub enum Code {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "PARAM_ERROR")]
    ParamError,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
}

#[derive(Serialize)]
pub struct Response<T> {
    code: Code,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: Code::Ok,
            message: String::from("ok"),
            data,
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: Code::NotFound,
            message: "not found".to_string(),
            data: None,
        }
    }

    pub fn params_error(message: String) -> Self {
        Self {
            code: Code::ParamError,
            message,
            data: None,
        }
    }

    pub fn internal_error(message: String) -> Self {
        Self {
            code: Code::InternalError,
            message,
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
