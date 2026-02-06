use std::fmt::Display;


#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
    Parse(serde_json::Error),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "Error: {}", e),
            Self::Parse(e) => write!(f, "Fail to parse response: {}", e),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(value)
    }
}
