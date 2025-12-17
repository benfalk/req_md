use super::AddressString;
use ::reqmd_http as http;

/// # Global HTTP Defaults
///
/// Provides default values for HTTP requests.  This
/// is normally populated from markdown frontmatter
/// under the `http` key.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GlobalHttpDefaults {
    pub server: AddressString,
    pub headers: http::Headers,
    pub query: http::QueryString,
}

impl GlobalHttpDefaults {
    /// create a [request factory] from the global defaults
    ///
    /// ```rust
    /// # use reqmd_markdown::ast::GlobalHttpDefaults;
    /// use ::reqmd_http as http;
    ///
    /// let defaults = GlobalHttpDefaults {
    ///     server: "https://api.example.com".parse().unwrap(),
    ///     headers: http::Headers::from_iter([("foo", "bar")]),
    ///     query: http::QueryString::from_iter([("baz", "qux")]),
    /// };
    ///
    /// let request = defaults
    ///     .factory()
    ///     .post("/v1/resource")
    ///     .build();
    ///
    /// assert_eq!(
    ///     request.build_url().as_str(),
    ///     "https://api.example.com/v1/resource?baz=qux"
    /// );
    /// assert_eq!(request.method, http::Method::Post);
    /// assert_eq!(request.headers.first("foo"), Some("bar"));
    /// ```
    ///
    /// [request factory]: http::RequestFactory
    /// ---
    pub fn factory(&self) -> http::RequestFactory {
        http::Request::factory()
            .address(self.server.address().clone())
            .with_headers(self.headers.clone())
            .with_query_string(self.query.clone())
            .build()
    }
}
