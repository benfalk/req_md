use crate::{BodyData, Error, Position, parsing::ParseContext};
use ::markdown::mdast::{self, Node};
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
    /// The heading in the markdown document found directly
    /// above the http code block.  The `#` and leading
    /// whitespace are trimmed.
    pub title: Option<String>,

    /// Any text found between the heading and the http code
    /// block is treated as a description.
    pub description: Option<String>,

    /// Parsed method from the beginning of the http
    /// code block.
    pub method: http::Method,

    /// just the path portion of the request line
    pub path: http::Path,

    /// data dextracted from the query string of the
    /// request line.
    pub query: http::QueryString,

    /// data extracted from the headers of the http code block
    pub headers: http::Headers,

    /// the next code block immediately following the http code
    /// block is treated as the body of the request ( if any ).
    pub body: BodyData,

    /// the range from which all of this data was extracted
    pub position: Position,
}

impl HttpData {
    pub(crate) fn try_collect(ctx: &ParseContext<'_>) -> Result<Vec<Self>, Error> {
        let mut data_set = Vec::new();
        let mut iter = ctx.root.children.iter().peekable();
        let mut prior_heading = None;

        while let Some(child) = iter.next() {
            match child {
                Node::Code(block) if is_http_code(block) => {
                    let mut data = parser::parse(&block.value)?;
                    data.position = Position::try_from(block)?;

                    if let Some(Node::Code(block)) = iter.peek()
                        && !is_http_code(block)
                    {
                        iter.next(); // consuming peeked block for body
                        data.body = BodyData::from(block.clone());
                        data.position.extend(&Position::try_from(block)?);
                    }

                    if let Some(heading) = prior_heading.take() {
                        let position = Position::try_from(heading)?;
                        let title = position
                            .find_substring(ctx.input)?
                            .trim_start_matches('#')
                            .trim();
                        data.title = Some(title.into());

                        if let Some(range) = position.range_between(&data.position) {
                            let desc = ctx.input[range].trim();
                            if !desc.is_empty() {
                                data.description = Some(desc.into());
                            }
                        }

                        data.position.extend(&position);
                    }

                    data_set.push(data);
                }
                Node::Heading(heading) => {
                    prior_heading = Some(heading);
                }
                _ => continue,
            }
        }

        Ok(data_set)
    }
}

fn is_http_code(code: &mdast::Code) -> bool {
    code.lang.as_deref() == Some("http")
}

mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;
    use crate::support::fixtures::post_widget_parse_context as ctx;

    #[rstest::rstest]
    fn collect_from_mdast_works(ctx: ParseContext<'static>) {
        let data = HttpData::try_collect(&ctx).unwrap();
        assert_eq!(data.len(), 1);
        let http = data.first().unwrap();
        dbg!(&http);
        assert_eq!(http.method, http::Method::Post);
        assert_eq!(http.path, http::Path::from("/widgets"));
        assert_eq!(http.query.first("foo"), Some("bar"));
        assert_eq!(http.query.first("rofl"), Some("copter"));
        assert_eq!(http.headers.first("authorization"), Some("Bearer abcd1234"));
        assert_eq!(http.body.lang.as_deref(), Some("json"));
        assert_eq!(http.body.meta.as_deref(), Some("http-body"));
        assert_eq!(http.title.as_deref(), Some("Post Widgets"));
        assert_eq!(
            http.description.as_deref(),
            Some("I've often wondered what this text is called")
        );
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
            http.position,
            Position {
                start: Point {
                    line: 8,
                    column: 1,
                    offset: 80
                },
                end: Point {
                    line: 24,
                    column: 4,
                    offset: 305
                },
            }
        );
    }
}
