use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeedError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
