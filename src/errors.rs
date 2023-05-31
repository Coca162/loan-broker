use poem::{http::StatusCode, IntoResponse, Response, error::ResponseError};
use poem_openapi::ApiResponse;
use thiserror::Error;
use tracing::{event, span, Level};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to use database")]
    Database(#[from] sqlx::Error),
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> Response
    {
        let span = span!(Level::DEBUG, "error_handling");
        let _entered = span.enter();

        match self {
            Self::Database(_) => {
                event!(Level::ERROR, %self, "Received unexpected error");
                (
                    self.status(),
                    "An unexpected error has occurred",
                )
                    .into_response()
            },
        }
    }
}
