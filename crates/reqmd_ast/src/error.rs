/// # Reqmd Markdown Error
///
/// represents all possible errors that can
/// occur within the Reqmd Markdown crate.
///
#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    #[error("Markdown parsing error: {0}")]
    Parse(String),

    #[error("Expected root node, found: {found:?}")]
    MissingRoot { found: Box<::markdown::mdast::Node> },

    #[error("Invalid YAML with error: {message}")]
    InvalidYaml { input: String, message: String },

    #[error("Required position missing from node: {node:?}")]
    MissingPosition { node: Box<::markdown::mdast::Node> },

    #[error("Unable to read position offset: {position:?}")]
    InvalidOffset {
        data: Box<str>,
        position: Box<crate::Position>,
    },
}

impl From<::markdown::message::Message> for Error {
    fn from(value: ::markdown::message::Message) -> Self {
        Error::Parse(value.to_string())
    }
}

impl From<::serde_saphyr::Error> for Error {
    fn from(value: ::serde_saphyr::Error) -> Self {
        Error::Parse(value.to_string())
    }
}
