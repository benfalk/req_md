use crate::{Error, GlobalHttpDefaults, Position, parsing::ParseContext};
use ::markdown::mdast;

/// # Meta Data AST
///
/// This is information is extracted from the [FrontMatter]
/// of a markdown document.  The `http` key if present is
/// directly derserialized as [global defaults].
///
/// [FrontMatter]: mdast::Node::Yaml
/// [global defaults]: crate::GlobalHttpDefaults
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct MetaData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub http: GlobalHttpDefaults,
    pub position: Option<Position>,
}

impl TryFrom<&ParseContext<'_>> for MetaData {
    type Error = Error;

    fn try_from(value: &ParseContext) -> Result<Self, Self::Error> {
        #[derive(::serde::Deserialize, Default)]
        #[serde(default)]
        struct Data {
            title: Option<String>,
            description: Option<String>,
            http: GlobalHttpDefaults,
        }

        let maybe_yaml = value.root.children.first().and_then(|node| match node {
            mdast::Node::Yaml(yaml) => Some((&yaml.position, &yaml.value)),
            _ => None,
        });

        if let Some((position, yaml_str)) = maybe_yaml {
            let data: Data = ::serde_saphyr::from_str(yaml_str).map_err(|err| {
                Error::InvalidYaml {
                    input: yaml_str.to_owned(),
                    message: err.to_string(),
                }
            })?;
            Ok(Self {
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
    use crate::Point;
    use crate::support::fixtures::post_widget_parse_context as ctx;

    #[rstest::rstest]
    fn parses_from_mardown_ast(ctx: ParseContext<'static>) {
        let default = MetaData::try_from(&ctx).unwrap();
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
