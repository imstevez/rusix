use crate::api::response::*;
use actix_web::ResponseError;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;

pub fn handle_err<B: MessageBody>(
    res: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();
    let message = res.error().map(|e| e.to_string()).unwrap_or_default();
    let err_res = match res.status() {
        StatusCode::NOT_FOUND => Error::not_found(),
        StatusCode::BAD_REQUEST => Error::request_error(message),
        _ => Error::internal_error(message),
    };
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        req,
        err_res.error_response().map_into_right_body(),
    )))
}
