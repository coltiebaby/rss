pub mod client;
pub mod core;
pub mod connector;

use tungstenite::http;

pub type LCUResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("league of legends is not running")]
    AppNotRunning,
    #[error("failed to make client: {0}")]
    HttpClientError(String),
    #[error("http request creation failed: {0}")]
    Request(#[from] http::Error),
}
