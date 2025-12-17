use super::{HttpData, MetaData, Position};
use crate::Error;
use ::markdown::mdast;

/// # Markdown Document AST
///
/// Data extracted from a markdown document that
/// provides information needed to produce HTTP
/// requests.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Document {
    pub meta: MetaData,
    pub requests: Vec<HttpData>,
    pub position: Option<Position>,
}

impl Document {
    pub(crate) fn from_mdast(node: &mdast::Node) -> Result<Self, Error> {
        Ok(Document {
            meta: MetaData::from_mdast(node)?,
            requests: HttpData::collect_from_mdast(node)?,
            position: node.position().map(Position::from),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Point;
    use crate::support::fixtures::*;
    use ::reqmd_http as http;

    #[rstest::rstest]
    fn parse_document(post_widget_mdast: mdast::Node) {
        let doc = Document::from_mdast(&post_widget_mdast).unwrap();
        assert_eq!(doc.requests.len(), 1);
        assert!(doc.meta.title.is_none());
        assert!(doc.meta.description.is_none());
        assert!(doc.meta.http.query.is_empty());
        assert_eq!(doc.meta.http.headers.len(), 1);

        assert_eq!(
            doc.meta.http.headers.first("content-type"),
            Some("application/json")
        );

        assert_eq!(
            doc.position.unwrap(),
            Position {
                start: Point {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                end: Point {
                    line: 25,
                    column: 1,
                    offset: 306
                },
            }
        );

        let http = doc.requests.first().unwrap();
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
