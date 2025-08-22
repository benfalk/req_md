use super::{Response, ResponseBody, Status};
use crate::header::HeaderLine;

/// # Response Builder
///
/// Creates http responses with a builder pattern.
///
/// ```rust
/// # use reqmd_http::{Response, Status};
/// let resp = Response::builder()
///     .status(200)
///     .header("Content-Type", "application/json")
///     .body_text(r#"{"message":"Hello, world!"}"#)
///     .build();
///
/// assert_eq!(resp.status, Status(200));
/// assert_eq!(resp.headers.first("Content-Type"), Some("application/json"));
/// assert_eq!(resp.body.text(), Some(r#"{"message":"Hello, world!"}"#));
/// ```
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ResponseBuilder {
    response: Response,
}

impl ResponseBuilder {
    pub(super) fn new(response: Response) -> Self {
        Self { response }
    }

    pub fn status(mut self, status: u16) -> Self {
        self.response.status = Status(status);
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.response.headers.add(key, value);
        self
    }

    pub fn multiple_headers<I, H>(mut self, header_lines: I) -> Self
    where
        H: Into<HeaderLine>,
        I: IntoIterator<Item = H>,
    {
        self.response.headers.insert_many(header_lines);
        self
    }

    pub fn body(mut self, body: ResponseBody) -> Self {
        self.response.body = body;
        self
    }

    pub fn body_none(mut self) -> Self {
        self.response.body = ResponseBody::None;
        self
    }

    pub fn body_text(mut self, text: impl Into<String>) -> Self {
        self.response.body = ResponseBody::Text(text.into());
        self
    }

    pub fn body_binary(mut self, binary: Vec<u8>) -> Self {
        self.response.body = ResponseBody::Binary(binary);
        self
    }

    pub fn build(self) -> Response {
        self.response
    }
}
