use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::{fmt::Display, sync::PoisonError};

#[derive(Debug)]
pub enum Error {
    Pprof(pprof::Error),
    Poison(String),
    General(String),
}

impl From<pprof::Error> for Error {
    fn from(err: pprof::Error) -> Self {
        Error::Pprof(err)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Error::Poison(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Pprof(err) => write!(f, "{err}"),
            Error::Poison(err) => write!(f, "{err}"),
            Error::General(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Pprof(_) | Self::General(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Poison(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
