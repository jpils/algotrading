#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Float parse error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("HTTP request failed: {0}")]
    HTTP(#[from] reqwest::Error),

    #[error("Generic: (0)")] 
    Generic(String)
}
