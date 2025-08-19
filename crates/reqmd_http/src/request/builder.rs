use super::{Body, Method, Path, Request};
use crate::address::{Host, Scheme};
use std::marker::PhantomData;

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
    pub(super) fn new(request: Request) -> Self {
        Self {
            request,
            target: PhantomData,
        }
    }

    pub fn address_port(mut self, port: u16) -> Self {
        self.request.address.port = Some(port);
        self
    }

    pub fn address_host<H>(mut self, host: H) -> Self
    where
        H: Into<Host>,
    {
        self.request.address.host = host.into();
        self
    }

    pub fn address_scheme<S>(mut self, scheme: S) -> Self
    where
        S: Into<Scheme>,
    {
        self.request.address.scheme = scheme.into();
        self
    }

    pub fn post<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Post;
        self.request.path = path.into();
        self
    }

    pub fn get<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Get;
        self.request.path = path.into();
        self
    }

    pub fn delete<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Delete;
        self.request.path = path.into();
        self
    }

    pub fn put<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Put;
        self.request.path = path.into();
        self
    }

    pub fn patch<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.method = Method::Patch;
        self.request.path = path.into();
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

    pub fn build(self) -> Target {
        Target::from(self.request)
    }
}
