use crate::api::response;
use actix_web::Responder;
use actix_web::body::{MessageBody};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;

pub fn handle_err<B: MessageBody>(
    res: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();
    let message = res.error().map(|e| e.to_string()).unwrap_or_default();
    let nrs = match res.status() {
        StatusCode::NOT_FOUND => response::Response::<i32>::not_found().respond_to(&req),
        StatusCode::BAD_REQUEST => {
            response::Response::<i32>::params_error(message).respond_to(&req)
        }
        _ => response::Response::<i32>::internal_error(message).respond_to(&req),
    };
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        req,
        nrs.map_into_boxed_body().map_into_right_body(),
    )))
}
