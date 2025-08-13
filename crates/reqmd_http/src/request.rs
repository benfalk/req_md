pub use ::url::Url;
pub use body::Body;
pub use method::Method;
pub use path::Path;
pub use query_string::QueryString;

use crate::header::Headers;

/// # Request
///
/// Encaspsulates the request data for an HTTP request.
///
/// ```rust
/// # use reqmd_http::request::{Body, Method, Request};
/// let req = Request::post("http://example.com/widget")
///     .expect("Failed to parse URL")
///     .header("Content-Type", "application/json")
///     .header("Authorization", "Bearer token")
///     .query_param("trace_id", "12345")
///     .text_body(r#"{"name": "Widget", "price": 19.99}"#)
///     .build();
///
/// assert_eq!(req.method, Method::Post);
/// assert_eq!(req.url.as_str(), "http://example.com/widget?trace_id=12345");
/// assert_eq!(req.headers.len(), 2);
/// assert_eq!(req.headers[0].key, "Content-Type");
/// assert_eq!(req.headers[0].value, "application/json");
/// assert_eq!(req.headers.first_value_for("authorization"), Some("Bearer token"));
/// assert_eq!(
///     req.body,
///     Body::Text(r#"{"name": "Widget", "price": 19.99}"#.to_string())
/// );
/// ```
/// ---
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub headers: Headers,
    pub body: Body,
}

/// # Request Builder
///
/// Offers a fluent interface to build an HTTP request.
///
#[derive(Debug, Clone, PartialEq)]
pub struct RequestBuilder {
    method: Method,
    url: Url,
    headers: Headers,
    body: Body,
}

mod body;
mod builder_methods;
mod factory;
mod method;
mod path;
mod query_string;

#[cfg(test)]
mod tests;
