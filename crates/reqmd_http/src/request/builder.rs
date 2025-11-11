use super::{Method, Path, QueryParameter, QueryString, Request, RequestBody};
use crate::address::{Address, Host, Scheme};
use crate::header::{HeaderLine, Headers};
use std::marker::PhantomData;

/// # Request Builder
///
/// Allows for construction and modification of [request]
/// instances as it's primary function; however, it can
/// build directly to anything that can be built from a
/// request.
///
/// ```rust
/// # use reqmd_http::{RequestBuilder, Method, Request, Scheme, Host};
/// let req: Request = RequestBuilder::default()
///     .address_scheme(Scheme::Https)
///     .address_host(Host::parse("example.com").unwrap())
///     .address_port(8080)
///     .header("Content-Type", "application/json")
///     .query_param("key", "foo")
///     .method(Method::Post)
///     .path("/api/v1/resource")
///     .body_text("my-resource")
///     .build();
///
/// assert_eq!(req.address.port, Some(8080));
/// assert_eq!(req.address.host.to_string(), "example.com");
/// assert_eq!(req.address.scheme, Scheme::Https);
/// assert_eq!(req.method, Method::Post);
/// assert_eq!(req.path.as_str(), "/api/v1/resource");
/// assert_eq!(req.query.first("key"), Some("foo"));
/// assert_eq!(req.headers.first("content-type"), Some("application/json"));
/// assert_eq!(req.body.text(), Some("my-resource"));
/// ```
/// [request]: Request
/// ---
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RequestBuilder<Target = Request>
where
    Target: From<Request>,
{
    request: Request,
    target: PhantomData<Target>,
}

impl<Target> RequestBuilder<Target>
where
    Target: From<Request>,
{
    #[doc(hidden)]
    pub(super) fn new(request: Request) -> Self {
        Self {
            request,
            target: PhantomData,
        }
    }

    /// sets the [address] for the request
    ///
    /// due to the failable nature of the address host
    /// this allows for setting the whole address at once
    /// from an outside failable parse of it.
    ///
    /// ```rust
    /// # use reqmd_http::{Request, Address, Scheme};
    /// # fn main() -> Result<(), reqmd_http::Error> {
    /// let example_address = Address::parse("https://example.com:8080")?;
    /// let req = Request::builder()
    ///     .address(example_address)
    ///     .get("/index.html")
    ///     .build();
    ///
    /// assert_eq!(req.address.host.to_string(), "example.com");
    /// assert_eq!(req.address.scheme, Scheme::Https);
    /// assert_eq!(req.address.port, Some(8080));
    /// assert_eq!(req.build_url().as_str(), "https://example.com:8080/index.html");
    /// # Ok(()) }
    /// ```
    /// [address]: Address
    /// ---
    pub fn address(mut self, address: Address) -> Self {
        self.request.address = address;
        self
    }

    /// set server address port number
    ///
    /// Will automatically pick a port if not provided based
    /// on the [scheme] of the request at run-time and will
    /// report as none.
    /// ```rust
    /// # use reqmd_http::{Request, Scheme};
    /// let http_default = Request::builder()
    ///     .address_scheme(Scheme::Http)
    ///     .build();
    ///
    /// assert!(http_default.address.port.is_none());
    /// assert_eq!(http_default.build_url().as_str(), "http://localhost/");
    ///
    /// let https_default = Request::builder()
    ///     .address_scheme(Scheme::Https)
    ///     .build();
    ///
    /// assert!(https_default.address.port.is_none());
    /// assert_eq!(https_default.build_url().as_str(), "https://localhost/");
    ///
    /// let http_custom = Request::builder()
    ///     .address_scheme(Scheme::Http)
    ///     .address_port(3001)
    ///     .build();
    ///
    /// assert_eq!(http_custom.address.port, Some(3001));
    /// assert_eq!(http_custom.build_url().as_str(), "http://localhost:3001/");
    ///
    /// let https_custom = Request::builder()
    ///     .address_scheme(Scheme::Https)
    ///     .address_port(3002)
    ///     .build();
    ///
    /// assert_eq!(https_custom.address.port, Some(3002));
    /// assert_eq!(https_custom.build_url().as_str(), "https://localhost:3002/");
    /// ```
    /// [scheme]: Scheme
    /// ---
    pub fn address_port(mut self, port: u16) -> Self {
        self.request.address.port = Some(port);
        self
    }

    /// set server address [host]
    ///
    /// This is an infailable process; unfortunately parsing a
    /// host name is not.  Setting it once at boot in [factories]
    /// would allow preventing the need to parse it all of the time.
    ///
    /// ```rust
    /// # use reqmd_http::{Host, Request, Error};
    /// # fn main() -> Result<(), Error> {
    /// let host = Host::parse("example.com")?;
    /// let req = Request::builder()
    ///     .address_host(host)
    ///     .build();
    ///
    /// assert_eq!(req.build_url().as_str(), "http://example.com/");
    /// # Ok(()) }
    /// ```
    /// [host]: Host
    /// [factories]: Request::factory
    /// ---
    pub fn address_host<H>(mut self, host: H) -> Self
    where
        H: Into<Host>,
    {
        self.request.address.host = host.into();
        self
    }

    /// set address [scheme]
    ///
    /// ```rust
    /// # use reqmd_http::{Scheme, Request};
    /// ```
    /// [scheme]: Scheme
    /// ---
    pub fn address_scheme<S>(mut self, scheme: S) -> Self
    where
        S: Into<Scheme>,
    {
        self.request.address.scheme = scheme.into();
        self
    }

    /// POST path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let req = Request::builder()
    ///     .header("Content-Type", "application/json")
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
    pub fn post<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Post;
        self.request.path = path.into();
        self
    }

    /// GET path
    /// ```rust
    /// # use reqmd_http::{Method, Request};
    /// let request = Request::builder()
    ///     .get("/api/v1/resource")
    ///     .build();
    ///
    /// assert_eq!(request.method, Method::Get);
    /// assert_eq!(request.path.as_str(), "/api/v1/resource");
    /// assert!(request.query.is_empty());
    /// ```
    /// ---
    pub fn get<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Get;
        self.request.path = path.into();
        self
    }

    /// DELETE path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let req = Request::builder()
    ///     .header("Authorization", "Bearer some-token")
    ///     .delete("/foo/123")
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Delete);
    /// assert_eq!(req.headers.first("authorization"), Some("Bearer some-token"));
    /// assert_eq!(req.path.as_str(), "/foo/123");
    /// assert!(req.body.is_empty());
    /// ```
    /// ---
    pub fn delete<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Delete;
        self.request.path = path.into();
        self
    }

    /// PUT path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let req = Request::builder()
    ///     .header("Content-Type", "application/json")
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
    pub fn put<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Put;
        self.request.path = path.into();
        self
    }

    /// PATCH path
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let req = Request::builder()
    ///     .header("Authorization", "ApiKey $rofl$")
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
    pub fn patch<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Patch;
        self.request.path = path.into();
        self
    }

    /// set the request [method]
    ///
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let req = Request::builder()
    ///     .method(Method::Options)
    ///     .path("/api/*")
    ///     .build();
    ///
    /// assert_eq!(req.method, Method::Options);
    /// assert_eq!(req.path.as_str(), "/api/*");
    /// ```
    /// [method]: Method
    /// ---
    pub fn method(mut self, method: Method) -> Self {
        self.request.method = method;
        self
    }

    /// set the request [path]
    ///
    /// ```rust
    /// # use reqmd_http::{Request, Method};
    /// let req = Request::builder().path("/foo").build();
    ///
    /// assert_eq!(req.path.as_str(), "/foo");
    /// ```
    /// [path]: Path
    /// ---
    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.path = path.into();
        self
    }

    /// add a [query] parameter key/value pair
    ///
    /// ```rust
    /// # use reqmd_http::Request;
    ///
    /// let req = Request::builder()
    ///     .path("/api")
    ///     .query_param("key", "value")
    ///     .query_param("another", "value2")
    ///     .build();
    ///
    /// assert_eq!(req.query.first("key"), Some("value"));
    /// assert_eq!(req.query.first("another"), Some("value2"));
    /// assert_eq!(
    ///     req.build_url().as_str(),
    ///     "http://localhost/api?key=value&another=value2"
    /// );
    /// ```
    /// [query]: crate::request::QueryString
    /// ---
    pub fn query_param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.request.query.add(key, value);
        self
    }

    /// add multiple [query] parameters
    ///
    /// ```rust
    /// # use reqmd_http::Request;
    /// let req = Request::builder()
    ///     .path("/search")
    ///     .multiple_query_params([
    ///         ("q", "rust"),
    ///         ("page", "1"),
    ///     ])
    ///     .build();
    ///
    /// assert_eq!(req.query.first("q"), Some("rust"));
    /// assert_eq!(req.query.first("page"), Some("1"));
    /// assert_eq!(
    ///     req.build_url().as_str(),
    ///     "http://localhost/search?q=rust&page=1"
    /// );
    /// ```
    /// [query]: crate::request::QueryString
    /// ---
    pub fn multiple_query_params<I, Q>(mut self, params: I) -> Self
    where
        Q: Into<QueryParameter>,
        I: IntoIterator<Item = Q>,
    {
        self.request.query.insert_many(params);
        self
    }

    /// sets the entire [query string] for the request
    ///
    /// ```rust
    /// # use reqmd_http::{Request, QueryString};
    /// let query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("bing", "bong"),
    /// ]);
    ///
    /// let req = Request::builder()
    ///     .with_query_string(query)
    ///     .build();
    ///
    /// assert_eq!(req.query.first("foo"), Some("bar"));
    /// assert_eq!(req.query.first("bing"), Some("bong"));
    /// ```
    /// [query string]: crate::request::QueryString
    /// ---
    pub fn with_query_string(mut self, query: QueryString) -> Self {
        self.request.query = query;
        self
    }

    /// add a [header] key/value pair
    ///
    /// ```rust
    /// #  use reqmd_http::Request;
    /// let req = Request::builder()
    ///     .header("Content-Type", "application/json")
    ///     .header("Authorization", "Bearer my-token")
    ///     .build();
    ///
    /// assert_eq!(req.headers.first("content-type"), Some("application/json"));
    /// assert_eq!(req.headers.first("authorization"), Some("Bearer my-token"));
    /// ```
    /// [header]: crate::header::Headers
    /// ---
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.request.headers.add(key, value);
        self
    }

    /// add multiple [header] key/value pairs
    ///
    /// ```rust
    /// # use reqmd_http::Request;
    /// let req = Request::builder()
    ///     .multiple_headers([
    ///         ("Authorization", "Bearer token"),
    ///         ("Content-Type", "application/json"),
    ///     ])
    ///     .build();
    ///
    /// assert_eq!(req.headers.first("authorization"), Some("Bearer token"));
    /// assert_eq!(req.headers.first("content-type"), Some("application/json"));
    /// ```
    /// [header]: crate::header::Headers
    /// ---
    pub fn multiple_headers<I, H>(mut self, header_lines: I) -> Self
    where
        H: Into<HeaderLine>,
        I: IntoIterator<Item = H>,
    {
        self.request.headers.insert_many(header_lines);
        self
    }

    /// sets the entire [headers] collection for the request
    ///
    /// ```rust
    /// # use reqmd_http::{Request, Headers};
    /// let headers = Headers::from_iter([
    ///     ("Authorization", "Bearer token"),
    ///     ("Content-Type", "application/json"),
    /// ]);
    ///
    /// let req = Request::builder()
    ///     .with_headers(headers)
    ///     .build();
    ///
    /// assert_eq!(req.headers.first("authorization"), Some("Bearer token"));
    /// assert_eq!(req.headers.first("content-type"), Some("application/json"));
    /// ```
    /// [headers]: crate::header::Headers
    /// ---
    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.request.headers = headers;
        self
    }

    /// Sets a [body] for the request.
    ///
    /// ```rust
    /// # use reqmd_http::{Request, RequestBody};
    /// let req = Request::builder()
    ///     .post("/api/v1/resource")
    ///     .body(RequestBody::Binary(vec![1, 2, 3, 4]))
    ///     .build();
    ///
    /// assert_eq!(req.body.data(), &[1, 2, 3, 4]);
    /// ```
    /// [body]: crate::RequestBody
    /// ---
    pub fn body<B>(mut self, body: B) -> Self
    where
        B: Into<RequestBody>,
    {
        self.request.body = body.into();
        self
    }

    /// Prepares a [text body] for the request.
    ///
    /// ```rust
    /// #  use reqmd_http::Request;
    /// let req = Request::builder()
    ///     .post("/api/v1/resource")
    ///     .body_text(r#"{"name":"foo"}"#)
    ///     .build();
    ///
    /// assert_eq!(req.body.text(), Some(r#"{"name":"foo"}"#));
    /// ```
    /// [text body]: crate::RequestBody::Text
    /// ---
    pub fn body_text<B>(mut self, body: B) -> Self
    where
        B: Into<String>,
    {
        self.request.body = RequestBody::Text(body.into());
        self
    }

    /// Prepares a [binary body] for the request.
    ///
    /// ```rust
    /// # use reqmd_http::Request;
    /// let req = Request::builder()
    ///     .post("/api/v1/resource")
    ///     .body_binary([1, 2, 3, 4])
    ///     .build();
    ///
    /// assert_eq!(req.body.data(), &[1, 2, 3, 4]);
    /// ```
    /// [binary body]: crate::RequestBody::Binary
    /// ---
    pub fn body_binary<B>(mut self, body: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        self.request.body = RequestBody::Binary(body.into());
        self
    }

    /// Prepares an [empty body] for the request.
    ///
    /// This is the default state of a request body
    /// and you only need to call this if you want to
    /// change the body back to empty.
    ///
    /// ```rust
    /// # use reqmd_http::{Request, RequestBody};
    /// let req = Request::builder()
    ///     .get("/api/v1/resource")
    ///     .body_none()
    ///     .build();
    ///
    /// assert!(req.body.is_empty());
    /// assert_eq!(req.body, RequestBody::None);
    /// assert_eq!(req.body.text(), None);
    /// assert_eq!(req.body.data(), &[]);
    /// assert_eq!(req.body.len(), 0);
    /// ```
    /// [empty body]: crate::RequestBody::None
    /// ---
    pub fn body_none(mut self) -> Self {
        self.request.body = RequestBody::None;
        self
    }

    /// builds the request or target type
    ///
    /// The target type must implement [From<Request>]
    /// and is normally just [Request] itself.  It is
    /// also use by [factories] to build directly
    ///
    /// [factories]: Request::factory
    /// ---
    pub fn build(self) -> Target {
        Target::from(self.request)
    }
}
