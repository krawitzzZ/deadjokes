use actix_web::{dev::ServiceResponse, error, middleware::ErrorHandlerResponse, ResponseError};

use crate::response::Error;

#[tracing::instrument(name = "api:error_handler:handle_data")]
pub fn handle_data<T: std::error::Error + 'static>(err: T) -> error::Error {
    let reason = match err.to_string() {
        s if s.to_lowercase().contains("deserialize error") => {
            String::from("Failed to deserialize JSON payload")
        }
        s => s,
    };
    let response = Error::BadRequest(Some(reason)).error_response();

    error::InternalError::from_response(err, response).into()
}

pub fn handle_generic<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    if res.response().error().is_some() {
        Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    } else {
        let error = Error::from(res.status());
        let error = error::InternalError::from_response(error.clone(), error.error_response());

        Ok(ErrorHandlerResponse::Response(
            res.error_response(error).map_into_right_body(),
        ))
    }
}
