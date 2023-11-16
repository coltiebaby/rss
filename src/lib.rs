pub mod client;

use thiserror::Error;

pub type LCUResult<T> = Result<T, LCUError>;

#[derive(Debug, Error)]
pub enum LCUError {
    #[error("league of legends is not running")]
    AppNotRunning,
    #[error("failed to make client: {0}")]
    HttpClientError(String),
}
