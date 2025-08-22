pub use url::Host;

use crate::error::Error;
use ::url::Url;

/// # HTTP Address
///
/// Describes the access address of an HTTP server,
/// including host, scheme, and port.
///
/// ```rust
/// # use reqmd_http::{Address, Scheme, Host};
/// let host = Host::parse("example.com").expect("valid host");
/// let address = Address::builder(|addr|{
///     addr.scheme(Scheme::Https)
///         .host(host)
///         .port(8080);
/// });
///
/// assert_eq!(address.host.to_string(), "example.com");
/// assert_eq!(address.scheme, Scheme::Https);
/// assert_eq!(address.port, Some(8080));
/// assert_eq!(address.build_url().as_str(), "https://example.com:8080/");
/// ```
/// ---
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Address {
    pub host: Host,
    pub scheme: Scheme,
    pub port: Option<u16>,
}

impl Address {
    /// # Parse from URL
    ///
    /// ```rust
    /// # use reqmd_http::{Address, Scheme};
    /// # fn main() -> Result<(), reqmd_http::Error> {
    /// let address = Address::parse("https://example.com:8080")?;
    /// assert_eq!(address.host.to_string(), "example.com");
    /// assert_eq!(address.scheme, Scheme::Https);
    /// assert_eq!(address.port, Some(8080));
    /// # Ok(()) }
    /// ```
    /// ---
    pub fn parse(url: &str) -> Result<Self, Error> {
        let parsed_url = Url::parse(url)?;
        Ok(Self::from_url(&parsed_url))
    }

    /// # Build URL
    /// ```rust
    /// # use reqmd_http::{Address, Scheme, Host};
    /// let host = Host::parse("example.com").expect("valid host");
    /// let address = Address::builder(|addr|{
    ///     addr.scheme(Scheme::Https)
    ///         .host(host)
    ///         .port(8080);
    /// });
    ///
    /// assert_eq!(address.build_url().as_str(), "https://example.com:8080/");
    /// ```
    /// ---
    pub fn build_url(&self) -> Url {
        let mut url = Url::parse(&format!("{}://{}", self.scheme.as_str(), self.host))
            .expect("Failed to parse URL");
        url.set_port(self.port).expect("Failed to set port");
        url
    }

    /// # From URL
    /// ```rust
    /// # use reqmd_http::{Address, Scheme};
    /// let url = "https://example.com:8080".parse::<url::Url>().expect("valid URL");
    /// let address = Address::from_url(&url);
    ///
    /// assert_eq!(address.host.to_string(), "example.com");
    /// assert_eq!(address.scheme, Scheme::Https);
    /// assert_eq!(address.port, Some(8080));
    /// ````
    /// ---
    pub fn from_url(url: &Url) -> Self {
        Self::builder(|builder| {
            if let Some(host) = url.host() {
                builder.host(host.to_owned());
            }

            if let Some(port) = url.port() {
                builder.port(port);
            }

            if let Some(scheme) = Scheme::parse_str(url.scheme()) {
                builder.scheme(scheme);
            }
        })
    }

    /// # Build Address
    /// ```rust
    /// # use reqmd_http::{Address, Scheme, Host};
    /// let host = Host::parse("127.0.0.1").expect("valid host");
    /// let address = Address::builder(|addr|{
    ///     addr.scheme(Scheme::Https)
    ///         .host(host)
    ///         .port(8080);
    /// });
    ///
    /// assert_eq!(address.host.to_string(), "127.0.0.1");
    /// assert_eq!(address.scheme, Scheme::Https);
    /// assert_eq!(address.port, Some(8080));
    /// ```
    /// ---
    pub fn builder<F>(builder_fn: F) -> Self
    where
        F: FnOnce(&mut AddressBuilder<'_>),
    {
        let mut address = Self::default();
        let mut builder = AddressBuilder {
            address: &mut address,
        };
        builder_fn(&mut builder);
        address
    }
}

impl Default for Address {
    fn default() -> Self {
        Self {
            host: Host::parse("localhost").expect("default host should be valid"),
            scheme: Scheme::default(),
            port: None,
        }
    }
}

/// HTTP scheme, either HTTP or HTTPS.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Scheme {
    #[default]
    Http,
    Https,
}

impl Scheme {
    /// # As String Representation
    /// ```rust
    /// # use reqmd_http::Scheme;
    /// assert_eq!(Scheme::Http.as_str(), "http");
    /// assert_eq!(Scheme::Https.as_str(), "https");
    /// ```
    /// ---
    pub const fn as_str(&self) -> &str {
        match self {
            Scheme::Http => "http",
            Scheme::Https => "https",
        }
    }

    /// # Parse Scheme String
    /// ```rust
    /// # use reqmd_http::Scheme;
    /// assert_eq!(Scheme::parse_str("http"), Some(Scheme::Http));
    /// assert_eq!(Scheme::parse_str("https"), Some(Scheme::Https));
    /// assert_eq!(Scheme::parse_str("ftp"), None);
    /// ```
    /// ---
    pub fn parse_str(string: &str) -> Option<Self> {
        match string {
            value if value.eq_ignore_ascii_case("http") => Some(Scheme::Http),
            value if value.eq_ignore_ascii_case("https") => Some(Scheme::Https),
            _ => None,
        }
    }
}

pub struct AddressBuilder<'a> {
    address: &'a mut Address,
}

impl AddressBuilder<'_> {
    /// # Set Host
    pub fn host<H>(&mut self, host: H) -> &mut Self
    where
        H: Into<Host>,
    {
        self.address.host = host.into();
        self
    }

    /// # Set Port
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.address.port = Some(port);
        self
    }

    /// # Set Scheme
    pub fn scheme<S>(&mut self, scheme: S) -> &mut Self
    where
        S: Into<Scheme>,
    {
        self.address.scheme = scheme.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_default() {
        let address = Address::default();
        assert_eq!(address.host, Host::parse("localhost").unwrap());
        assert_eq!(address.scheme, Scheme::Http);
        assert!(address.port.is_none());
    }
}
