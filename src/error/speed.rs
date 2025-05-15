use super::ErrorResponse;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

pub type SpeedResult<T> = std::result::Result<T, SpeedError>;

#[derive(thiserror::Error, Debug)]
pub enum SpeedError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    #[error("Failed to extract CSRF token: {0}")]
    CsrfToken(String),
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Selector parse error: {0}")]
    Selector(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("Validation failed: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Invalid region code: {0}")]
    InvalidRegion(String),
}
impl IntoResponse for SpeedError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Validation(_) => {
                tracing::info!(error = %self, kind = ?self, "Client error occurred");
                StatusCode::BAD_REQUEST
            }
            Self::Authentication(_) => {
                tracing::error!(error = %self, kind = ?self, "Authentication error occurred");
                StatusCode::UNAUTHORIZED
            }
            _ => {
                tracing::error!(error = %self, kind = ?self, "Internal server error occurred");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        ErrorResponse {
            status,
            message: (status != StatusCode::INTERNAL_SERVER_ERROR).then(|| self.to_string()),
        }
        .into_response()
    }
}
