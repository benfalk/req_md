#![allow(dead_code, unused_imports, unused_variables)]

use url::Host;

use super::{Body, Headers, Method, Path, QueryString, Request, RequestBuilder, Url};
use crate::address::{Address, Scheme};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestBuilderFactory {
    server_address: Address,
    method: Method,
    path: Path,
    query: QueryString,
    headers: Headers,
    body: Body,
}

impl RequestBuilderFactory {
    pub fn new<F>(builder_fn: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        let mut factory = Default::default();
        builder_fn(&mut factory);
        factory
    }

    pub fn address<F>(&mut self, builder_fn: F) -> &mut Self
    where
        F: FnOnce(&mut AddressFactory<'_>),
    {
        let mut factory = AddressFactory {
            address: &mut self.server_address,
        };

        builder_fn(&mut factory);
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<Path>,
    {
        self.path = path.into();
        self
    }

    pub fn query_param<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query.add(key, value);
        self
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.add(key, value);
        self
    }

    pub fn body_text<B>(&mut self, body: B) -> &mut Self
    where
        B: Into<String>,
    {
        self.body = Body::Text(body.into());
        self
    }
}

pub struct AddressFactory<'a> {
    address: &'a mut Address,
}

impl AddressFactory<'_> {
    pub fn host<H>(&mut self, host: H) -> &mut Self
    where
        H: Into<Host>,
    {
        self.address.host = host.into();
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Self {
        self.address.port = Some(port);
        self
    }

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
    fn full_build_works() {
        let factory = RequestBuilderFactory::new(|req| {
            req.address(|addr| {
                addr.scheme(Scheme::Https).port(8080);
            })
            .method(Method::Post)
            .path("/api/v1/resource")
            .query_param("key", "value")
            .header("Content-Type", "application/json")
            .body_text(r#"{"name": "Test"}"#);
        });

        assert_eq!(factory.server_address.scheme, Scheme::Https);
        assert_eq!(factory.server_address.port, Some(8080));
        assert_eq!(factory.server_address.host.to_string(), "localhost");
        assert_eq!(factory.method, Method::Post);
        assert_eq!(factory.path.as_str(), "/api/v1/resource");
        assert_eq!(factory.query.first_value_for("key"), Some("value"));
        assert_eq!(
            factory.headers.first_value_for("content-type"),
            Some("application/json")
        );
        assert_eq!(factory.body, Body::Text(r#"{"name": "Test"}"#.into()));
    }
}
