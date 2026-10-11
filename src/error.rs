//! The one error type rendering can fail with.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("cairo error: {0}")]
    Cairo(#[from] cairo::Error),
    #[error("could not read shadow pixels for blurring: {0}")]
    Borrow(#[from] cairo::BorrowError),
}
