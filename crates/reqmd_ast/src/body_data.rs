use super::Position;
use ::markdown::mdast::Code;
use ::reqmd_http as http;

/// # Body Data
///
/// Represents the body of a request and is normally
/// populated by a code block directly following a
/// `http` language marked code block.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BodyData {
    /// Populated by the contents of a code block
    pub content: http::RequestBody,

    /// String label of the code block if any
    pub lang: Option<String>,

    /// String following the language label of a code block
    pub meta: Option<String>,

    /// Where in the source the code block can be found.
    /// If this is generated not from source will be None.
    pub position: Option<Position>,
}

impl From<Code> for BodyData {
    fn from(code_block: Code) -> Self {
        BodyData {
            content: http::RequestBody::Text(code_block.value),
            lang: code_block.lang,
            meta: code_block.meta,
            position: code_block.position.map(Position::from),
        }
    }
}
