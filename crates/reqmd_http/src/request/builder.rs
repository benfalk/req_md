use super::{Body, Method, Path, Request};
use crate::address::{Host, Scheme};
use std::marker::PhantomData;

/// # Request Builder
///
/// Allows for construction and modification of [request]
/// instances as it's primary function; however, it can
/// build directly to anything that can be built from a
/// request.
///
/// ```rust
/// # use reqmd_http::request::{RequestBuilder, Method, Request};
/// # use reqmd_http::address::{Scheme, Host};
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

    /// set server address port number
    ///
    /// Will automatically pick a port if not provided based
    /// on the [scheme] of the request at run-time and will
    /// report as none.
    /// ```rust
    /// # use reqmd_http::{request::Request, address::Scheme};
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
    ///
    /// [scheme]: Scheme
    /// ---
    pub fn address_port(mut self, port: u16) -> Self {
        self.request.address.port = Some(port);
        self
    }

    /// set server address port
    ///
    /// This is an infailable process; unfortunately parsing a
    /// host name is not.  Setting it once at boot in [factories]
    /// would allow preventing to parse it all of the time.
    ///
    /// ```rust
    /// # use reqmd_http::{address::Host, request::Request, error::Error};
    /// # fn main() -> Result<(), Error> {
    /// let host = Host::parse("example.com")?;
    /// let req = Request::builder()
    ///     .address_host(host)
    ///     .build();
    ///
    /// assert_eq!(req.build_url().as_str(), "http://example.com/");
    /// # Ok(()) }
    /// ```
    ///
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
    /// # use reqmd_http::{address::Scheme, request::Request};
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
    /// # use reqmd_http::request::{Request, Method};
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
    /// # use reqmd_http::request::{Method, Request};
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
    /// # use reqmd_http::request::{Request, Method};
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
    /// # use reqmd_http::request::{Request, Method};
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
    /// # use reqmd_http::request::{Request, Method};
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
    /// # use reqmd_http::request::{Request, Method};
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
    /// # use reqmd_http::request::{Request, Method};
    /// let req = Request::builder().path("/foo").build();
    ///
    /// assert_eq!(req.path.as_str(), "/foo");
    /// ```
    ///
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
    /// # use reqmd_http::request::Request;
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
    ///
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

    /// add a [header] key/value pair
    ///
    /// ```rust
    /// #  use reqmd_http::request::Request;
    /// let req = Request::builder()
    ///     .header("Content-Type", "application/json")
    ///     .header("Authorization", "Bearer my-token")
    ///     .build();
    ///
    /// assert_eq!(req.headers.first("content-type"), Some("application/json"));
    /// assert_eq!(req.headers.first("authorization"), Some("Bearer my-token"));
    /// ```
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.request.headers.add(key, value);
        self
    }

    pub fn body_text<B>(mut self, body: B) -> Self
    where
        B: Into<String>,
    {
        self.request.body = Body::Text(body.into());
        self
    }

    pub fn build(self) -> Target {
        Target::from(self.request)
    }
}
