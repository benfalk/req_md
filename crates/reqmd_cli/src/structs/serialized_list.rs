use reqmd_core as core;
use reqmd_http as http;
use reqmd_markdown::ast;
use serde::ser::SerializeSeq;

/// # Serialized List
///
/// Meant to output a simple list of serialized
/// [MdRequest] objects without the base markdown
/// ast data that is attached to it.  This also
/// includes the location of each request object
/// sourced from the markdown.
///
/// [MdRequest]: core::MdRequest
/// [ast]
#[derive(Debug)]
pub struct SerializedList<'a>(pub &'a core::MdRequestList);

impl ::serde::ser::Serialize for SerializedList<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        #[derive(::serde::Serialize)]
        struct SimpleRequest<'a> {
            title: Option<&'a str>,
            description: Option<&'a str>,
            request: &'a http::Request,
            position: &'a ast::Position,
        }
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for req in self.0.iter() {
            let simple = SimpleRequest {
                title: req.title.as_deref(),
                description: req.description.as_deref(),
                request: &req.request,
                position: &req.data.position,
            };
            seq.serialize_element(&simple)?;
        }
        seq.end()
    }
}
