use crate::MdRequest;
use ::reqmd_http::{self as http, Client as _};
use ::reqwest::Client;
use ::std::time::Duration;

/// # HTTP Client
///
/// This struct provides HTTP client functionality for ReqMD
/// and is available when the `http-client` feature is enabled.
///
/// ## Construction via Builder
///
/// ```rust
/// # use ::reqmd_core::HttpClient;
/// use ::std::time::Duration;
///
/// // With a timeout
/// let client = HttpClient::builder()
///     .timeout(Duration::from_secs(10))
///     .build();
///
/// // With an optional timeout
/// let client = HttpClient::builder()
///     .maybe_timeout(Some(Duration::from_secs(5)))
///     .build();
///
/// // No timeout at all
/// let client = HttpClient::builder().build();
/// ```
///
/// ---
#[derive(Clone)]
pub struct HttpClient {
    http: Client,
}

#[bon::bon]
impl HttpClient {
    #[builder]
    pub fn new(timeout: Option<Duration>) -> Self {
        let mut builder = Client::builder();
        if let Some(duration) = timeout {
            builder = builder.timeout(duration);
        }
        let http = builder.build().expect("Failed to build HTTP client");
        Self { http }
    }

    /// # Send MD Request
    ///
    /// Convenience method to send an [Markdown request] as a
    /// whole to avoid needing to extract the inner HTTP request
    /// as well as using the [client] trait.
    ///
    /// [Markdown request]: MdRequest
    /// [client]: http::Client
    /// ---
    pub async fn send_md_request(
        &self,
        request: &MdRequest,
    ) -> Result<http::Response, http::Error> {
        self.http.send(&request.request).await
    }
}

impl http::Client for HttpClient {
    #[inline]
    async fn send(
        &self,
        request: &http::Request,
    ) -> Result<http::Response, http::Error> {
        self.http.send(request).await
    }
}
