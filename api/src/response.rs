use actix_web::{body::BoxBody, error, http::StatusCode, HttpResponse, HttpResponseBuilder};

use app::QueryResult;

pub type Response<T> = Result<Success<T>, Error>;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T: serde::Serialize> {
    current_page: u64,
    total_pages: u64,
    total_count: u64,
    items: Vec<T>,
}

impl<U: Into<T>, T: serde::Serialize> From<QueryResult<U>> for PaginatedResponse<T> {
    fn from(result: QueryResult<U>) -> Self {
        let items: Vec<T> = result.items.into_iter().map(|i| i.into()).collect();
        PaginatedResponse {
            // pagination starts with 0, so we have to add 1 for the response
            current_page: result.current_page + 1,
            total_pages: result.total_pages,
            total_count: result.total_count,
            items,
        }
    }
}

pub enum Success<T: serde::Serialize> {
    Ok(T),
    Created(T),
}

impl<T: serde::Serialize> Success<T> {
    fn json(code: StatusCode, response: T) -> HttpResponse {
        HttpResponseBuilder::new(code).json(response)
    }
}

impl<T: serde::Serialize> actix_web::Responder for Success<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Success::Ok(res) => Success::json(StatusCode::OK, res),
            Success::Created(res) => Success::json(StatusCode::CREATED, res),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error, serde::Serialize)]
pub enum Error {
    #[error("Bad Request")]
    BadRequest(Option<String>),
    #[error("Not found")]
    NotFound,
    #[error("Conflict")]
    Conflict,
    #[error("Internal Server Error")]
    InternalError,
}

impl Error {
    fn json_response(status: StatusCode, reason: Option<String>) -> HttpResponse {
        let message = status.canonical_reason().unwrap_or("Error"); // should never happen
        HttpResponseBuilder::new(status).json(serde_json::json!({
            "message": message,
            "reason": reason,
            "status": status.as_u16(),
        }))
    }
}

impl From<StatusCode> for Error {
    fn from(code: StatusCode) -> Self {
        match code {
            StatusCode::BAD_REQUEST => Error::BadRequest(None),
            StatusCode::NOT_FOUND => Error::NotFound,
            StatusCode::INTERNAL_SERVER_ERROR => Error::InternalError,
            c if c.is_client_error() => Error::BadRequest(None),
            c if c.is_server_error() => Error::InternalError,
            _ => Error::InternalError,
        }
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self.clone() {
            Error::BadRequest(reason) => Error::json_response(StatusCode::BAD_REQUEST, reason),
            Error::NotFound => Error::json_response(StatusCode::NOT_FOUND, None),
            Error::Conflict => Error::json_response(StatusCode::CONFLICT, None),
            Error::InternalError => Error::json_response(StatusCode::INTERNAL_SERVER_ERROR, None),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Conflict => StatusCode::CONFLICT,
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn ok<T: serde::Serialize>(response: T) -> Result<Success<T>, Error> {
    Ok(Success::Ok(response))
}

pub fn created<T: serde::Serialize>(response: T) -> Result<Success<T>, Error> {
    Ok(Success::Created(response))
}

pub fn _bad_request<T: serde::Serialize>(reason: Option<String>) -> Result<Success<T>, Error> {
    Err(Error::BadRequest(reason))
}

pub fn not_found<T: serde::Serialize>() -> Result<Success<T>, Error> {
    Err(Error::NotFound)
}
