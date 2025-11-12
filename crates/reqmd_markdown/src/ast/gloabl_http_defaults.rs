use super::AddressString;
use ::reqmd_http as http;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GlobalHttpDefaults {
    pub server: AddressString,
    pub headers: http::Headers,
    pub query: http::QueryString,
}

impl GlobalHttpDefaults {
    pub fn factory(&self) -> http::RequestFactory {
        http::Request::factory()
            .address(self.server.address().clone())
            .with_headers(self.headers.clone())
            .with_query_string(self.query.clone())
            .build()
    }
}
