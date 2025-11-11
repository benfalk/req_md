use super::{BodyData, Position};
use crate::Error;
use ::markdown::mdast::Node;
use ::reqmd_http as http;

/// # HTTP Data
///
/// Data extracted from a markdown document `code`
/// block that is tagged with the language of `http`.
/// This format is mostly the same as a raw http
/// request.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct HttpData {
    pub method: http::Method,
    pub path: http::Path,
    pub query: http::QueryString,
    pub headers: http::Headers,
    pub body: BodyData,
    pub position: Option<Position>,
}

impl HttpData {
    pub(crate) fn collect_from_mdast(node: &Node) -> Result<Vec<Self>, Error> {
        if node.children().is_none() {
            return Ok(vec![]);
        }

        let mut data_set = Vec::new();
        let mut iter = node.children().unwrap().iter().peekable();

        while let Some(child) = iter.next() {
            if let Node::Code(code_block) = child
                && code_block.lang.as_deref() == Some("http")
            {
                let mut data = parser::parse(&code_block.value)?;
                let mut position = code_block.position.as_ref().map(Position::from);

                if let Some(Node::Code(code)) = iter.peek()
                    && code.lang.as_deref() != Some("http")
                {
                    iter.next(); // consuming peeked block for body
                    data.body = BodyData::from(code.clone());
                    if let Some(pos) = &mut position
                        && let Some(body) = &data.body.position
                    {
                        pos.extend(body);
                    }
                }

                data.position = position;
                data_set.push(data);
            }
        }

        Ok(data_set)
    }
}

mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Point;
    use crate::support::fixtures::*;

    #[rstest::rstest]
    fn collect_from_mdast_works(post_widget_mdast: Node) {
        let data = HttpData::collect_from_mdast(&post_widget_mdast).unwrap();
        assert_eq!(data.len(), 1);
        let http = data.first().unwrap();
        assert_eq!(http.method, http::Method::Post);
        assert_eq!(http.path, http::Path::from("/widgets"));
        assert_eq!(http.query.first("foo"), Some("bar"));
        assert_eq!(http.query.first("rofl"), Some("copter"));
        assert_eq!(http.headers.first("authorization"), Some("Bearer abcd1234"));
        assert_eq!(http.body.lang.as_deref(), Some("json"));
        assert_eq!(http.body.meta.as_deref(), Some("http-body"));
        assert_eq!(
            http.body.position.as_ref(),
            Some(&Position {
                start: Point {
                    line: 19,
                    column: 1,
                    offset: 232
                },
                end: Point {
                    line: 24,
                    column: 4,
                    offset: 305
                },
            })
        );
        assert_eq!(
            http.body.content.text(),
            Some("{\n  \"name\": \"XFox\",\n  \"desc\": \"Wonderful widget!\"\n}")
        );
        assert_eq!(
            http.position.as_ref(),
            Some(&Position {
                start: Point {
                    line: 12,
                    column: 1,
                    offset: 142
                },
                end: Point {
                    line: 24,
                    column: 4,
                    offset: 305
                },
            })
        );
    }
}
