use crate::{Entity, Id};
use ::reqmd_http as http;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct HttpInputs {
    pub id: Id,
    pub address: Option<http::Address>,
    pub method: http::Method,
    pub path: http::Path,
    pub query: http::QueryString,
    pub headers: http::Headers,
    pub body: http::RequestBody,
}

impl Entity for HttpInputs {
    fn id(&self) -> Id {
        self.id
    }
}
