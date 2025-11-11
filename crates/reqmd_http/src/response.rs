pub use body::ResponseBody;
pub use builder::ResponseBuilder;
pub use status::Status;

use crate::header::Headers;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Response {
    pub status: Status,
    pub headers: Headers,
    pub body: ResponseBody,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new(Response::default())
    }
}

mod body;
mod builder;
mod status;
