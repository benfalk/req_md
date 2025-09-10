use ::reqmd_http as http;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct HttpDefaults {
    pub address: http::Address,
    pub query: http::QueryString,
    pub headers: http::Headers,
}

impl HttpDefaults {
    pub fn generate_builder(&self) -> http::RequestBuilder {
        http::RequestBuilder::default()
            .address(self.address.clone())
            .multiple_query_params(self.query.clone())
            .multiple_headers(self.headers.clone())
    }
}
