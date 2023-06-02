use poem::{error::ResponseError, http::StatusCode, IntoResponse, Response};

use thiserror::Error;
use tracing::{event, span, Level};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Custom: ({0}) {1}")]
    Custom(StatusCode, &'static str),
    #[error("Failed to use database: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Failed to request something from spookvooper.com: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        match self {
            Error::Custom(status_code, _) => *status_code,
            Error::Database(_) | Error::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> Response {
        let span = span!(Level::DEBUG, "error_handling");
        let _entered = span.enter();

        match self {
            Error::Custom(status_code, text) => (*status_code, *text).into_response(),
            Error::Database(err) => {
                event!(Level::ERROR, %err);
                (self.status(), "An unexpected error has occurred").into_response()
            }
            Error::Reqwest(err) => {
                event!(Level::ERROR, %err);
                (
                    self.status(),
                    "Failed to request something from spookvooper.com",
                )
                    .into_response()
            }
        }
    }
}
