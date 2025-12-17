use crate::Error;
use crate::ast::Document;
use ::markdown::{Constructs, ParseOptions};

/// Parse Markdown Document
///
/// Scans a markdown formatted string and produces
/// a [document]. This document contains all of the
/// [http request] information for use.
///
/// [document]: Document
/// [http request]: ::reqmd_http
/// ---
pub fn parse_markdown(input: &str) -> Result<Document, Error> {
    let options = ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Constructs::default()
        },
        ..Default::default()
    };
    let mdast = ::markdown::to_mdast(input, &options)?;
    Document::from_mdast(&mdast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Point, Position};
    use crate::support::fixtures::*;
    use ::reqmd_http as http;

    #[rstest::rstest]
    fn test_parse_markdown(post_widget_md: &str) -> Result<(), Error> {
        let doc = parse_markdown(post_widget_md)?;
        assert_eq!(doc.requests.len(), 1);
        assert_eq!(
            doc.position.as_ref(),
            Some(&Position {
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
            })
        );

        let req = doc.requests.first().unwrap();
        assert_eq!(req.method, http::Method::Post);
        assert_eq!(req.path.as_str(), "/widgets");
        assert_eq!(req.query.first("foo"), Some("bar"));
        assert_eq!(req.query.first("rofl"), Some("copter"));
        assert_eq!(req.headers.first("authorization"), Some("Bearer abcd1234"));
        assert_eq!(
            req.body.content.text(),
            Some("{\n  \"name\": \"XFox\",\n  \"desc\": \"Wonderful widget!\"\n}")
        );
        assert_eq!(
            req.position.as_ref(),
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
        Ok(())
    }
}
