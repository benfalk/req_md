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
