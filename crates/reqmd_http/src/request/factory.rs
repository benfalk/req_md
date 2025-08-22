use super::{Method, Request, RequestBuilder};
use std::sync::Arc;

/// # Request Factory
///
/// This is a starting point for building HTTP requests with
/// a configured server address, method, path, query parameters.
/// Handy when you need to create multiple requests that all
/// require the same base configuration.
///
/// For more docs see [Request::factory]
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
    /// # use reqmd_http::{Request, RequestFactory, Method};
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
    /// # use reqmd_http::{Method, Request};
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
    /// # use reqmd_http::{Method, RequestFactory};
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

    /// POST path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let factory = Request::factory()
    ///     .header("Content-Type", "application/json")
    ///     .build();
    ///
    /// let req = factory
    ///     .post("/widgets")
    ///     .body_text(r#"{"name":"foo"}"#)
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Post);
    /// assert_eq!(req.headers.first("content-type"), Some("application/json"));
    /// assert_eq!(req.path.as_str(), "/widgets");
    /// assert_eq!(req.body.text(), Some(r#"{"name":"foo"}"#));
    /// ```
    /// ---
    pub fn post(&self, path: &str) -> RequestBuilder {
        self.request(Method::Post, path)
    }

    /// PUT path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let factory = Request::factory()
    ///     .header("Content-Type", "application/json")
    ///     .build();
    ///
    /// let req = factory
    ///     .put("/foo")
    ///     .body_text("fiz-buz")
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Put);
    /// assert_eq!(req.headers.first("content-type"), Some("application/json"));
    /// assert_eq!(req.path.as_str(), "/foo");
    /// assert_eq!(req.body.text(), Some("fiz-buz"));
    /// ```
    /// ---
    pub fn put(&self, path: &str) -> RequestBuilder {
        self.request(Method::Put, path)
    }

    /// DELETE path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let factory = Request::factory()
    ///     .header("Authorization", "Bearer some-token")
    ///     .build();
    ///
    /// let req = factory
    ///     .delete("/foo/123")
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Delete);
    /// assert_eq!(req.headers.first("authorization"), Some("Bearer some-token"));
    /// assert_eq!(req.path.as_str(), "/foo/123");
    /// assert!(req.body.is_empty());
    /// ```
    /// ---
    pub fn delete(&self, path: &str) -> RequestBuilder {
        self.request(Method::Delete, path)
    }

    /// PATCH path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let factory = Request::factory()
    ///     .header("Authorization", "ApiKey $rofl$")
    ///     .build();
    ///
    /// let req = factory
    ///     .patch("/my/email")
    ///     .body_text("domain=example.com")
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Patch);
    /// assert_eq!(req.headers.first("authorization"), Some("ApiKey $rofl$"));
    /// assert_eq!(req.path.as_str(), "/my/email");
    /// assert_eq!(req.body.text(), Some("domain=example.com"));
    /// ```
    /// ---
    pub fn patch(&self, path: &str) -> RequestBuilder {
        self.request(Method::Patch, path)
    }
}

impl From<Request> for RequestFactory {
    fn from(request: Request) -> Self {
        Self::new(request)
    }
}
