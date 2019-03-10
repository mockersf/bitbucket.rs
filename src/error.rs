use std::error::Error as StdError;

impl StdError for Error {}
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidUrl { url } => write!(f, "Error converting to url '{}'", url),
            Error::Http { url, error } => write!(f, "HTTP error on '{}': {}", url, error),
            Error::ErrorResponse {
                url,
                status_code,
                message,
            } => write!(f, "Error on '{}': {} - {}", url, status_code, message),
            Error::Deserialization(err) => write!(f, "Deserialization error: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidUrl {
        url: String,
    },
    Http {
        url: String,
        error: reqwest::Error,
    },
    ErrorResponse {
        url: String,
        status_code: reqwest::StatusCode,
        message: String,
    },
    Deserialization(serde_json::error::Error),
}
