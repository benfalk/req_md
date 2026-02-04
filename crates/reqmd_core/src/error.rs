#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    AST(#[from] ::reqmd_markdown::Error),

    #[error("Error in Defaults Provider '{provider}': {source:?}")]
    DefaultProvider {
        provider: String,

        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Error in Factory Processor '{processor}': {source:?}")]
    FactoryProcessor {
        processor: String,

        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error(transparent)]
    Http(#[from] ::reqmd_http::Error),
}
