use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("ApiClientNetworkError: code: {0}, msg: {1}")]
    NetworkError(u16, String),
    #[error("ApiClientServerError: code: {0}, msg: {1}")]
    ServerError(u16, String),
    #[error("ApiClientClientError: code: {0}, msg: {1}")]
    ClientError(u16, String),
    #[error("ApiClientUnknownError: {0}")]
    UnknownError(String),
    #[error("SerdeQSError: {0}")]
    SerdeQSError(#[from] serde_qs::Error),
    #[error("SerdeJSONError: {0}")]
    SerdeJSONError(#[from] serde_json::Error),
}

impl From<reqwest::Error> for ApiClientError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            match status.as_u16() {
                400..=499 => ApiClientError::ClientError(status.as_u16(), err.to_string()),
                500..=599 => ApiClientError::ServerError(status.as_u16(), err.to_string()),
                _ => ApiClientError::NetworkError(status.as_u16(), err.to_string()),
            }
        } else {
            ApiClientError::UnknownError(err.to_string())
        }
    }
}

impl From<reqwest_middleware::Error> for ApiClientError {
    fn from(err: reqwest_middleware::Error) -> Self {
        if let Some(status) = err.status() {
            match status.as_u16() {
                400..=499 => ApiClientError::ClientError(status.as_u16(), err.to_string()),
                500..=599 => ApiClientError::ServerError(status.as_u16(), err.to_string()),
                _ => ApiClientError::NetworkError(status.as_u16(), err.to_string()),
            }
        } else {
            ApiClientError::UnknownError(err.to_string())
        }
    }
}

pub type ApiClientsResult<T> = Result<T, ApiClientError>;