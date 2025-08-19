pub use self::body::Body;
pub use self::builder::RequestBuilder;
pub use self::factory::RequestFactory;
pub use self::method::Method;
pub use self::path::Path;
pub use self::query_string::QueryString;

use crate::address::Address;
use crate::header::Headers;
use ::url::Url;

/// # Request
///
/// Encaspsulates the request data for an HTTP request.
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[readonly::make]
#[non_exhaustive]
pub struct Request {
    pub address: Address,
    pub method: Method,
    pub path: Path,
    pub query: QueryString,
    pub headers: Headers,
    pub body: Body,
}

impl Request {
    /// # Request Builder
    ///
    /// Starts a new request builder that can be used to
    /// create a [Request] instance.
    ///
    /// ```rust
    /// # use reqmd_http::request::{Request, Method, Body};
    /// let request = Request::builder()
    ///     .address_port(3000)
    ///     .method(Method::Post)
    ///     .path("/api/v1/resource")
    ///     .header("Content-Type", "application/json")
    ///     .body_text(r#"{"foo":"bar"}"#)
    ///     .build();
    ///
    /// assert_eq!(request.method, Method::Post);
    /// assert_eq!(request.path.as_str(), "/api/v1/resource");
    /// assert_eq!(request.headers.first("Content-Type"), Some("application/json"));
    /// assert_eq!(request.body.text(), Some(r#"{"foo":"bar"}"#));
    /// assert_eq!(
    ///     request.build_url().as_str(),
    ///     "http://localhost:3000/api/v1/resource"
    /// );
    /// ```
    /// ---
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new(Self::default())
    }

    /// # Request Factory
    ///
    /// Prepares a request factory that can be used to create
    /// a base configuration for multiple requests.
    ///
    /// ```rust
    /// # use reqmd_http::{
    /// #   request::{Request, RequestFactory, Method, Body},
    /// #   address::Scheme};
    /// let factory: RequestFactory = Request::factory()
    ///     .address_port(8080)
    ///     .address_scheme(Scheme::Https)
    ///     .header("Content-Type", "application/json")
    ///     .build();
    ///
    /// let request = factory
    ///     .post("/api/v1/resource")
    ///     .body_text(r#"{"foo":"bar"}"#)
    ///     .build();
    ///
    /// assert_eq!(request.method, Method::Post);
    /// assert_eq!(request.path.as_str(), "/api/v1/resource");
    /// assert_eq!(request.body, Body::Text(r#"{"foo":"bar"}"#.into()));
    /// assert_eq!(request.address.scheme, Scheme::Https);
    /// assert_eq!(request.address.port, Some(8080));
    /// assert_eq!(request.headers.first("Content-Type"), Some("application/json"));
    ///
    /// // Small and easy to clone around to start a new request
    /// assert_eq!(
    ///     std::mem::size_of::<RequestFactory>(),
    ///     std::mem::size_of::<usize>(),
    /// );
    /// ```
    /// ---
    pub fn factory() -> RequestBuilder<RequestFactory> {
        RequestBuilder::new(Request::default())
    }

    /// # Build URL
    ///
    /// Constructs a URL from the request's server address,
    /// path, and query parameters.
    ///
    /// ```rust
    /// # use reqmd_http::request::Request;
    /// let req = Request::builder()
    ///     .path("/search")
    ///     .query_param("q", "rust")
    ///     .build();
    ///
    /// assert_eq!(
    ///     req.build_url().as_str(),
    ///     "http://localhost/search?q=rust"
    /// );
    /// ```
    /// ---
    pub fn build_url(&self) -> Url {
        let mut url = self.address.build_url();
        url.set_path(self.path.as_str());
        if !self.query.is_empty() {
            let mut query_string = url.query_pairs_mut();
            for param in self.query.iter() {
                query_string.append_pair(&param.key, &param.value);
            }
            query_string.finish();
        };
        url
    }
}

mod body;
mod builder;
mod factory;
mod method;
mod path;
mod query_string;

#[cfg(test)]
mod tests;
