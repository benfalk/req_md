use super::{Body, Method, Path, Request};
use crate::address::{Address, Host, Scheme};
use ::url::Url;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub(super) fn new(request: Request) -> Self {
        Self { request }
    }

    pub fn address<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut AddressFactory<'_>),
    {
        let mut factory = AddressFactory {
            address: &mut self.request.server_address,
        };

        builder_fn(&mut factory);
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.request.method = method;
        self
    }

    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.path = path.into();
        self
    }

    pub fn query_param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.request.query.add(key, value);
        self
    }

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
}

impl From<RequestBuilder> for Request {
    fn from(builder: RequestBuilder) -> Self {
        builder.request
    }
}

pub struct AddressFactory<'a> {
    address: &'a mut Address,
}

impl AddressFactory<'_> {
    pub fn with_url(&mut self, url: &Url) -> &mut Self {
        if let Some(host) = url.host() {
            self.address.host = host.to_owned();
        }

        if let Some(port) = url.port() {
            self.address.port = Some(port);
        }

        if let Some(scheme) = Scheme::parse_str(url.scheme()) {
            self.address.scheme = scheme;
        }

        self
    }

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
