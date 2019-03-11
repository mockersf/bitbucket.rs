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

/// Errors that can be returned by the API client
#[derive(Debug)]
pub enum Error {
    /// Error parsing the requested URL
    InvalidUrl {
        /// url that was requested
        url: String,
    },
    /// HTTP error during request
    Http {
        /// url that was requested
        url: String,
        /// underlying error
        error: reqwest::Error,
    },
    /// Error response by Bitbucket
    ErrorResponse {
        /// url that was requested
        url: String,
        /// status code returned by Bitbucket
        status_code: reqwest::StatusCode,
        /// message provided by Bitbucket
        message: String,
    },
    /// Error deserializing Bitbucket response
    Deserialization(serde_json::error::Error),
}
