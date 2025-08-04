#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] ::url::ParseError),

    #[error(transparent)]
    ClientError(Box<dyn std::error::Error + Send + Sync>),
}
