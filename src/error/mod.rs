mod speed;
pub use speed::*;

// use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use reqwest::StatusCode;

#[derive(Debug)]
struct ErrorResponse {
    status: StatusCode,
    message: Option<String>,
}
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        match self.message {
            Some(msg) => (self.status, msg).into_response(),
            None => self.status.into_response(),
        }
    }
}
