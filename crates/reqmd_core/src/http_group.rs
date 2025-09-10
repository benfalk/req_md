use crate::{EntityCollection, HttpDefaults, HttpInputs, Id};
use reqmd_http as http;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HttpGroup {
    pub defaults: HttpDefaults,
    pub inputs: EntityCollection<HttpInputs>,
}

impl HttpGroup {
    pub fn generate_request(&self, id: &Id) -> Option<http::Request> {
        self.inputs.get(id).map(|input| {
            let mut builder = self.defaults.generate_builder();
            if let Some(ref address) = input.address {
                builder = builder.address(address.clone());
            }
            builder
                .method(input.method)
                .path(input.path.clone())
                .multiple_query_params(input.query.clone())
                .multiple_headers(input.headers.clone())
                .body(input.body.clone())
                .build()
        })
    }

    pub fn iter_requests(&self) -> impl Iterator<Item = (Id, http::Request)> {
        self.inputs.iter().map(move |input| {
            let mut builder = self.defaults.generate_builder();
            if let Some(ref address) = input.address {
                builder = builder.address(address.clone());
            }
            (
                input.id,
                builder
                    .method(input.method)
                    .path(input.path.clone())
                    .multiple_query_params(input.query.clone())
                    .multiple_headers(input.headers.clone())
                    .body(input.body.clone())
                    .build(),
            )
        })
    }
}
