use thiserror::Error;

#[derive(Error, Debug)]
pub enum TtsError {
    #[error("error in communicating with the API")]
    BadRequest,
    #[error("error in authentication with the API")]
    AuthError,
    #[error("too many requests sent over the quote")]
    TooManyRequest,
    #[error("media type not supported for query")]
    UnsupportedMediaType,
    #[error("error communicating with the api")]
    ConnectionError(#[from] reqwest::Error),
    #[error("unknown error communicating with the api")]
    UnknownConnectionError,
}
