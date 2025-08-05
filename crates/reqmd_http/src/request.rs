pub use ::url::Url;
pub use body::Body;
pub use method::Method;

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

mod builder_methods {
    use super::{Body, Headers, Method, Request, RequestBuilder, Url};
    use crate::error::Error;

    impl Request {
        /// # [Method::Get] Builder
        /// ```rust
        /// # use reqmd_http::request::{Request, Method};
        /// let req = Request::get("http://example.com")
        ///     .expect("Failed to parse URL")
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Get);
        /// ```
        pub fn get(url: &str) -> Result<RequestBuilder, Error> {
            Self::builder(Method::Get, url)
        }

        /// # [Method::Post] Builder
        /// ```rust
        /// # use reqmd_http::request::{Request, Method};
        /// let req = Request::post("http://example.com/widget")
        ///     .expect("Failed to parse URL")
        ///     .header("Content-Type", "application/json")
        ///     .text_body(r#"{"name": "Widget", "price": 19.99}"#)
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Post);
        /// ```
        pub fn post(url: &str) -> Result<RequestBuilder, Error> {
            Self::builder(Method::Post, url)
        }

        /// # [Method::Put] Builder
        /// ```rust
        /// # use reqmd_http::request::{Request, Method};
        /// let req = Request::put("http://example.com/resource/123")
        ///     .expect("Failed to parse URL")
        ///     .header("Content-Type", "application/json")
        ///     .text_body(r#"{"name": "Widget", "price": 19.99}"#)
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Put);
        /// ```
        pub fn put(url: &str) -> Result<RequestBuilder, Error> {
            Self::builder(Method::Put, url)
        }

        /// # [Method::Delete] Builder
        /// ```rust
        /// # use reqmd_http::request::{Request, Method};
        /// let req = Request::delete("http://example.com/resource/123")
        ///     .expect("Failed to parse URL")
        ///     .header("Authorization", "Bearer token")
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Delete);
        /// ```
        pub fn delete(url: &str) -> Result<RequestBuilder, Error> {
            Self::builder(Method::Delete, url)
        }

        /// # [Method::Patch] Builder
        /// ```rust
        /// # use reqmd_http::request::{Request, Method};
        /// let req = Request::patch("http://example.com/widget/123")
        ///     .expect("Failed to parse URL")
        ///     .header("Content-Type", "application/json")
        ///     .text_body(r#"{"name": "Updated Widget"}"#)
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Patch);
        /// ```
        pub fn patch(url: &str) -> Result<RequestBuilder, Error> {
            Self::builder(Method::Patch, url)
        }

        /// # [Method] Request Builder
        ///
        /// This is useful for cases where the method is
        /// not known at compile time.
        ///
        /// This is the only time a failure can occur, which
        /// is when the [Url] is invalid.
        ///
        /// ## Example
        ///
        /// ```rust
        /// # use reqmd_http::request::{Request, Method, Body};
        /// let req = Request::builder(Method::Get, "http://example.com")
        ///     .expect("Failed to parse URL")
        ///     .query_param("search", "markdown")
        ///     .header("Accept", "text/html")
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Get);
        /// assert_eq!(req.url.as_str(), "http://example.com/?search=markdown");
        /// assert_eq!(req.headers.first_value_for("Accept"), Some("text/html"));
        /// assert_eq!(req.body, Body::None);
        /// ```
        /// ---
        pub fn builder(method: Method, url: &str) -> Result<RequestBuilder, Error> {
            let url = Url::parse(url)?;
            Ok(RequestBuilder {
                method,
                url,
                headers: Headers::default(),
                body: Body::default(),
            })
        }
    }

    impl RequestBuilder {
        /// # adds query parameter to url
        /// ```rust
        /// # use reqmd_http::request::Request;
        /// let req = Request::get("http://foobar.com")
        ///     .expect("Failed to parse URL")
        ///     .query_param("q", "bar")
        ///     .query_param("q", "baz")
        ///     .query_param("z", "42%")
        ///     .build();
        ///
        /// assert_eq!(req.url.as_str(), "http://foobar.com/?q=bar&q=baz&z=42%25");
        /// ```
        /// ---
        pub fn query_param(mut self, key: &str, value: &str) -> Self {
            let mut query_pairs = self.url.query_pairs_mut();
            query_pairs.append_pair(key, value).finish();
            drop(query_pairs);
            self
        }

        /// # adds single header to request
        /// ```rust
        /// # use reqmd_http::request::Request;
        /// let req = Request::get("http://example.com")
        ///     .expect("Failed to parse URL")
        ///     .header("Accept", "application/json")
        ///     .build();
        ///
        /// assert_eq!(req.headers[0].key, "Accept");
        /// assert_eq!(req.headers[0].value, "application/json");
        /// ```
        /// ---
        pub fn header(mut self, key: &str, value: &str) -> Self {
            self.headers.add(key, value);
            self
        }

        /// # includes utf-8 text body
        /// ```rust
        /// # use reqmd_http::request::{Body, Request};
        /// let req = Request::post("http://example.com")
        ///     .expect("Failed to parse URL")
        ///     .header("Content-Type", "text/plain")
        ///     .text_body("Hello, world!")
        ///     .build();
        ///
        /// assert_eq!(req.body, Body::Text("Hello, world!".to_string()));
        /// ```
        /// ---
        pub fn text_body<T: Into<String>>(mut self, body: T) -> Self {
            self.body = Body::Text(body.into());
            self
        }

        /// # includes a binary body
        /// ```rust
        /// # use reqmd_http::request::{Body, Request};
        /// let req = Request::post("http://example.com")
        ///     .expect("Failed to parse URL")
        ///     .header("Content-Type", "application/octet-stream")
        ///     .binary_body(vec![1, 2, 3, 4])
        ///     .build();
        ///
        /// assert_eq!(req.body, Body::Binary(vec![1, 2, 3, 4]));
        /// ```
        /// ---
        pub fn binary_body<T: Into<Vec<u8>>>(mut self, body: T) -> Self {
            self.body = Body::Binary(body.into());
            self
        }

        /// # creates request from builder
        /// ```rust
        /// # use reqmd_http::request::{Body, Method, Request};
        /// let req = Request::get("http://example.com")
        ///     .expect("Failed to parse URL")
        ///     .query_param("q", "rust")
        ///     .header("Accept", "application/json")
        ///     .build();
        ///
        /// assert_eq!(req.method, Method::Get);
        /// assert_eq!(req.url.as_str(), "http://example.com/?q=rust");
        /// ```
        /// ---
        pub fn build(self) -> Request {
            self.into()
        }
    }

    impl From<RequestBuilder> for Request {
        fn from(builder: RequestBuilder) -> Self {
            Request {
                method: builder.method,
                url: builder.url,
                headers: builder.headers,
                body: builder.body,
            }
        }
    }
}
mod method {
    /// HTTP defines a set of request methods to indicate the purpose of
    /// the request and what is expected if the request is successful.
    /// Although they can also be nouns, these request methods are sometimes
    /// referred to as HTTP verbs. Each request method has its own semantics,
    /// but some characteristics are shared across multiple methods.
    ///
    /// These characteristics include:
    ///
    /// - `safe`:
    ///   A request method is considered safe if it doesn't
    ///   alter the state of the server
    ///
    /// - `idempotent`:
    ///   A request method is considered idempoten if the
    ///   intended effect on the server of making a single
    ///   request is the same as the effect of making several
    ///   identical requests
    ///
    /// - `cacheable`:
    ///   not all request methods can be cached per the specification
    ///
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Method {
        /// The GET HTTP method requests a representation of the specified
        /// resource. Requests using GET should only be used to request data
        /// and shouldn't contain a body.
        #[default]
        Get,

        /// The POST HTTP method sends data to the server. The type of the
        /// body of the request is indicated by the `Content-Type` header.
        Post,

        /// The PUT HTTP method creates a new resource or replaces a
        /// representation of the target resource with the request content.
        /// The difference between PUT and POST is that PUT is idempotent:
        /// calling it once is no different from calling it several times
        /// successively (there are no side effects).
        Put,

        /// The DELETE HTTP method asks the server to delete a specified
        /// resource. The DELETE method has no defined semantics for the
        /// message body, so this should be empty.
        Delete,

        /// The PATCH HTTP method applies partial modifications to a resource.
        ///
        /// PATCH is somewhat analogous to the "update" concept found in CRUD
        /// > In general, HTTP is different than CRUD, and the two should not
        /// > be confused.
        ///
        /// In comparison with PUT, a PATCH serves as a set of instructions
        /// for modifying a resource, whereas PUT represents a complete
        /// replacement of the resource. A PUT request is always idempotent
        /// (repeating the same request multiple times results in the resource
        /// remaining in the same state), whereas a PATCH request may not
        /// always be idempotent. For instance, if a resource includes an
        /// auto-incrementing counter, a PUT request will overwrite the
        /// counter (since it replaces the entire resource), but a PATCH
        /// request may not.
        Patch,

        /// The HEAD HTTP method requests the metadata of a resource in the
        /// form of headers that the server would have sent if the GET method
        /// was used instead. This method can be used in cases where a URL
        /// might produce a large download, for example, a HEAD request can
        /// read the Content-Length header to check the file size before
        /// downloading the file with a GET.
        Head,

        /// The CONNECT HTTP method requests that a proxy establish a HTTP
        /// tunnel to a destination server, and if successful, blindly
        /// forward data in both directions until the tunnel is closed.
        Connect,

        /// The OPTIONS HTTP method requests permitted communication options
        /// for a given URL or server. This can be used to test the allowed
        /// HTTP methods for a request, or to determine whether a request
        /// would succeed when making a CORS preflighted request. A client
        /// can specify a URL with this method, or an asterisk (*) to refer
        /// to the entire server.
        Options,

        /// The TRACE HTTP method performs a message loop-back test
        /// along the path to the target resource.
        Trace,
    }
}
mod body {
    /// # Body Content
    ///
    /// In HTTP messages, the content describes the 'information' conveyed
    /// in the message body (which follows the header section), after any
    /// message framing from HTTP/1.1 chunked transfer encoding has been
    /// removed. This was referred to as a "payload" in HTTP/1.1, but
    /// message "content" distinguishes from frame payloads in HTTP/2 and
    /// HTTP/3 where the data in a single frame could be header data,
    /// body data, or other control information.
    /// ---
    #[derive(Debug, Clone, PartialEq, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Body {
        /// No body is present in the request.
        #[default]
        None,

        /// Body is a binary representation of data.
        Binary(Vec<u8>),

        /// Body is a UTF-8 encoded text.
        Text(String),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    fn get_builder() {
        let req = Request::get("http://example.com")
            .expect("Failed to create GET request builder")
            .query_param("search", "markdown")
            .build();

        assert_eq!(req.method, Method::Get);
        assert_eq!(req.url.as_str(), "http://example.com/?search=markdown");
        assert!(req.headers.is_empty());
        assert_eq!(req.body, Body::None);
    }

    #[rstest::rstest]
    fn post_builder() {
        let req = Request::post("http://example.com")
            .expect("Failed to create POST request builder")
            .header("Content-Type", "text/plain")
            .text_body("Hello, world!")
            .build();

        assert_eq!(req.method, Method::Post);
        assert_eq!(req.url.as_str(), "http://example.com/");
        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].key, "Content-Type");
        assert_eq!(req.headers[0].value, "text/plain");
        assert_eq!(req.body, Body::Text("Hello, world!".to_string()));
    }

    #[rstest::rstest]
    fn put_builder() {
        let req = Request::put("http://example.com/resource")
            .expect("Failed to create PUT request builder")
            .header("Authorization", "Bearer token")
            .binary_body([42, 43, 44])
            .build();

        assert_eq!(req.method, Method::Put);
        assert_eq!(req.url.as_str(), "http://example.com/resource");
        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].key, "Authorization");
        assert_eq!(req.headers[0].value, "Bearer token");
        assert_eq!(req.body, Body::Binary(vec![42, 43, 44]));
    }

    #[rstest::rstest]
    fn delete_builder() {
        let req = Request::delete("http://example.com/resource/123")
            .expect("Failed to create DELETE request builder")
            .build();

        assert_eq!(req.method, Method::Delete);
        assert_eq!(req.url.as_str(), "http://example.com/resource/123");
        assert!(req.headers.is_empty());
        assert_eq!(req.body, Body::None);
    }
}
