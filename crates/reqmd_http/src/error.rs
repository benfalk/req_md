/// # HTTP Request/Response Error Handling
///
/// This enum defines the error types that can occur
/// when working with HTTP requests and responses.
///
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Some of the request processing uses the [url]
    /// crate under the hood, so this error captures
    /// instances where a valid URL could not be parsed.
    #[error(transparent)]
    ParseUrlError(#[from] ::url::ParseError),

    /// HTTP [client] errors that can occur.  There is no
    /// fixed client implementation, so this error is
    /// generic and can be used with any HTTP client.
    ///
    /// [client]: crate::client::Client
    #[error(transparent)]
    ClientError(Box<dyn std::error::Error + Send + Sync>),
}
