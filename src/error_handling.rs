use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum Errors {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Expired token")]
    ExpiredToken,
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}