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

#[cfg(test)]
mod tests {
    use super::*;
    use ::googletest::prelude::*;
    use ::markdown::unist::{Point as MarkdownPoint, Position as MarkdownPosition};

    #[rstest::fixture]
    fn code_block() -> Code {
        Code {
            value: "body content".to_string(),
            lang: Some("json".to_string()),
            meta: Some("meta info".to_string()),
            position: Some(MarkdownPosition {
                start: MarkdownPoint {
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                end: MarkdownPoint {
                    line: 3,
                    column: 1,
                    offset: 20,
                },
            }),
        }
    }

    #[rstest::rstest]
    #[googletest::gtest]
    fn code_block_converstion(code_block: Code) {
        let data = BodyData::from(code_block);
        let pos = data.position.as_ref().expect("a position");
        assert_that!(data.lang.as_deref(), eq(Some("json")));
        assert_that!(pos.start.line, eq(1));
        assert_that!(pos.start.column, eq(1));
        assert_that!(pos.start.offset, eq(0));
        assert_that!(pos.end.line, eq(3));
        assert_that!(pos.end.column, eq(1));
        assert_that!(pos.end.offset, eq(20));
    }
}
