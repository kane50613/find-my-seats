use std::sync::Arc;

#[derive(Debug, thiserror::Error, Clone)]
pub enum MyError {
    #[error("Request error: {0}")]
    RequestError(#[from] Arc<reqwest::Error>),
}

impl From<reqwest::Error> for MyError {
    fn from(error: reqwest::Error) -> Self {
        MyError::RequestError(Arc::new(error))
    }
}