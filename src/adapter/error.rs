#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("invalid game: {0}")]
    InvalidGame(String),
    #[error("failed to execute endpoint: {0}")]
    EndpointError(#[from] pandascore::endpoint::EndpointError),
}
