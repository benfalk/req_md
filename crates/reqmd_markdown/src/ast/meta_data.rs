use super::Position;
use crate::Error;
use ::markdown::mdast;
use ::reqmd_core::HttpDefaults;

/// # Meta Data AST
///
/// This is information is extracted from the [FrontMatter]
/// of a markdown document.  The `http` key if present is
/// directly derserialized as [HttpDefaults].
///
/// [FrontMatter]: mdast::Node::Yaml
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct MetaData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub http: HttpDefaults,
    pub position: Option<Position>,
}

impl MetaData {
    pub(crate) fn from_mdast(node: &mdast::Node) -> Result<Self, Error> {
        #[derive(::serde::Deserialize, Default)]
        #[serde(default)]
        struct Data {
            title: Option<String>,
            description: Option<String>,
            http: HttpDefaults,
        }

        let maybe_yaml = node
            .children()
            .and_then(|children| children.first())
            .and_then(|first_child| match first_child {
                mdast::Node::Yaml(yaml) => Some((&yaml.position, &yaml.value)),
                _ => None,
            });

        if let Some((position, yaml_str)) = maybe_yaml {
            let data: Data = ::serde_saphyr::from_str(yaml_str)
                .map_err(|err| Error::Parse(format!("frontmatter: {}", err)))?;
            Ok(MetaData {
                title: data.title,
                description: data.description,
                http: data.http,
                position: position.as_ref().map(Position::from),
            })
        } else {
            Ok(MetaData::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Point;
    use crate::support::fixtures::*;

    #[rstest::rstest]
    fn parses_from_mardown_ast(post_widget_mdast: mdast::Node) {
        let default = MetaData::from_mdast(&post_widget_mdast).unwrap();
        assert!(default.title.is_none());
        assert!(default.description.is_none());
        assert_eq!(
            default.http.headers.first("content-type"),
            Some("application/json")
        );
        assert_eq!(
            default.position,
            Some(Position {
                start: Point {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                end: Point {
                    line: 6,
                    column: 4,
                    offset: 78
                },
            })
        );
    }
}
