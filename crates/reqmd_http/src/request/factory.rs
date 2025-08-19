use super::{Method, Request, RequestBuilder};
use std::sync::Arc;

/// # Request Factory
///
/// This is a starting point for building HTTP requests with
/// a configured server address, method, path, query parameters.
/// Handy when you need to create multiple requests tha all
/// require the same base configuration.
///
/// For more docs see [crate::request::Request::factory].
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RequestFactory {
    request: Arc<Request>,
}

impl RequestFactory {
    #[doc(hidden)]
    pub(super) fn new(request: Request) -> Self {
        Self {
            request: Arc::new(request),
        }
    }

    /// # Start a new request builder
    ///
    /// Initializes a new request builder with the base configuration
    /// that was set in the factory.
    ///
    /// ```rust
    /// # use reqmd_http::request::{Request, RequestFactory, Method};
    /// let factory = Request::factory()
    ///     .address_port(3000)
    ///     .method(Method::Put)
    ///     .header("Content-Type", "application/json")
    ///     .query_param("secret", "key")
    ///     .build();
    ///
    /// let request = factory.builder()
    ///     .path("/api/v1/resource")
    ///     .body_text(r#"{"foo":"bar"}"#)
    ///     .build();
    ///
    /// assert_eq!(request.method, Method::Put);
    /// assert_eq!(request.path.as_str(), "/api/v1/resource");
    /// assert_eq!(request.headers.first("Content-Type"), Some("application/json"));
    /// assert_eq!(request.query.first("secret"), Some("key"));
    /// assert_eq!(request.body.text(), Some(r#"{"foo":"bar"}"#));
    /// assert_eq!(
    ///     request.build_url().as_str(),
    ///     "http://localhost:3000/api/v1/resource?secret=key"
    /// );
    /// ```
    /// ---
    pub fn builder(&self) -> RequestBuilder {
        RequestBuilder::new(self.request.as_ref().clone())
    }

    /// Starts a builder with the specified method and path.
    /// ```rust
    /// # use reqmd_http::request::{Method, Request};
    /// let factory = Request::factory().address_port(3000).build();
    /// let req = factory
    ///     .request(Method::Delete, "/api/v1/resource/42")
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Delete);
    /// assert_eq!(req.path.as_str(), "/api/v1/resource/42");
    /// assert_eq!(req.address.port, Some(3000));
    /// assert_eq!(
    ///     req.build_url().as_str(),
    ///     "http://localhost:3000/api/v1/resource/42"
    /// );
    /// ```
    /// ---
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.builder().method(method).path(path)
    }

    /// GET path
    /// ```rust
    /// # use reqmd_http::request::{Method, RequestFactory};
    /// let factory = RequestFactory::default();
    /// let request = factory.get("/api/v1/resource").build();
    ///
    /// assert_eq!(request.method, Method::Get);
    /// assert_eq!(request.path.as_str(), "/api/v1/resource");
    /// assert!(request.query.is_empty());
    /// ```
    /// ---
    pub fn get(&self, path: &str) -> RequestBuilder {
        self.request(Method::Get, path)
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.request(Method::Post, path)
    }

    pub fn put(&self, path: &str) -> RequestBuilder {
        self.request(Method::Put, path)
    }

    pub fn delete(&self, path: &str) -> RequestBuilder {
        self.request(Method::Delete, path)
    }

    pub fn patch(&self, path: &str) -> RequestBuilder {
        self.request(Method::Patch, path)
    }
}

impl From<Request> for RequestFactory {
    fn from(request: Request) -> Self {
        Self::new(request)
    }
}
