#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    /// Any error emanating from the core ReqMD library.
    #[error(transparent)]
    Core(#[from] ::reqmd_core::Error),

    #[error("Application Error: {message}")]
    Custom { message: String },
}

impl Error {
    /// Helper to create a custom application error with a message.
    pub fn custom<S: Into<String>>(message: S) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }
}
