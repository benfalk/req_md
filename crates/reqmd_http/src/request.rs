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
///
/// ```rust
/// # use reqmd_http::request::{Body, Method, Request};
/// ```
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[readonly::make]
#[non_exhaustive]
pub struct Request {
    pub server_address: Address,
    pub method: Method,
    pub path: Path,
    pub query: QueryString,
    pub headers: Headers,
    pub body: Body,
}

impl Request {
    /// Creates a new request builder.
    pub fn builder<F>(builder_fn: F) -> Self
    where
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        builder_fn(RequestBuilder::new(Self::default())).into()
    }

    /// # Request Factory
    ///
    /// Prepares a request factory that can be used to create
    /// a base configuration for multiple requests.
    ///
    /// ```rust
    /// # use reqmd_http::{
    /// #   request::{Request, RequestFactory, Method},
    /// #   address::Scheme};
    /// let factory: RequestFactory = Request::factory(|builder| {
    ///     builder.address(|addr| {
    ///         addr.scheme(Scheme::Https)
    ///             .port(8080);
    ///     })
    ///     .method(Method::Post)
    ///     .header("Content-Type", "application/json")
    /// });
    ///
    /// assert_eq!(
    ///     std::mem::size_of::<RequestFactory>(),
    ///     std::mem::size_of::<usize>(),
    /// );
    /// ```
    /// ---
    pub fn factory<F>(builder_fn: F) -> RequestFactory
    where
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        RequestFactory::new(Self::builder(builder_fn))
    }

    pub fn build_url(&self) -> Url {
        let mut url = self.server_address.build_url();
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
