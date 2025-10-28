use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiClientsError {
    #[error("NetworkError: code: {0}, msg: {1}")]
    NetworkError(u16, String),
    #[error("ServerError: code: {0}, msg: {1}")]
    ServerError(u16, String),
    #[error("ClientError: code: {0}, msg: {1}")]
    ClientError(u16, String),
    #[error("UnknownError: {0}")]
    UnknownError(String),
    #[error("UnexpectedResponse: {0}")]
    UnexpectedResponse(String),
    #[error("InvalidArgs: {0}")]
    InvalidArgs(String),
}

impl From<reqwest::Error> for ApiClientsError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            match status.as_u16() {
                400..=499 => ApiClientsError::ClientError(status.as_u16(), err.to_string()),
                500..=599 => ApiClientsError::ServerError(status.as_u16(), err.to_string()),
                _ => ApiClientsError::NetworkError(status.as_u16(), err.to_string()),
            }
        } else {
            ApiClientsError::UnknownError(err.to_string())
        }
    }
}

impl From<reqwest_middleware::Error> for ApiClientsError {
    fn from(err: reqwest_middleware::Error) -> Self {
        if let Some(status) = err.status() {
            match status.as_u16() {
                400..=499 => ApiClientsError::ClientError(status.as_u16(), err.to_string()),
                500..=599 => ApiClientsError::ServerError(status.as_u16(), err.to_string()),
                _ => ApiClientsError::NetworkError(status.as_u16(), err.to_string()),
            }
        } else {
            ApiClientsError::UnknownError(err.to_string())
        }
    }
}

pub type ApiClientResult<T> = Result<T, ApiClientsError>;
