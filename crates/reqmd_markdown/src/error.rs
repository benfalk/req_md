/// # Reqmd Markdown Error
///
/// represents all possible errors that can
/// occur within the Reqmd Markdown crate.
///
#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    #[error("Markdown parsing error: {0}")]
    Parse(String),
}

impl From<::markdown::message::Message> for Error {
    fn from(value: ::markdown::message::Message) -> Self {
        Error::Parse(value.to_string())
    }
}

impl From<::serde_json::Error> for Error {
    fn from(value: ::serde_json::Error) -> Self {
        Error::Parse(value.to_string())
    }
}

impl From<::serde_saphyr::Error> for Error {
    fn from(value: ::serde_saphyr::Error) -> Self {
        Error::Parse(value.to_string())
    }
}
