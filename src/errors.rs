use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

///
/// If desired, this is a single touchpoint to replace anyhow with, say,
/// thiserror or some other, custom Error type. Ensure the exported Result
/// uses the desired Error.
///

/**
 * App-internal type for all Errors.
 */
#[derive(Debug)]
pub struct AppError(anyhow::Error);

/**
 * App-internal type for all Results.
 */
pub(crate) type Result<T> = std::result::Result<T, AppError>;

// Inspired by:
// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
