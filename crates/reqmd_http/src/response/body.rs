#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResponseBody {
    /// No body is present in the response.
    #[default]
    None,

    /// Body is a UTF-8 encoded text.
    Text(String),

    /// Body is a binary representation of data.
    Binary(Vec<u8>),
}

impl ResponseBody {
    /// shorthand to reference body text if it is UTF-8 encoded.
    pub fn text(&self) -> Option<&str> {
        match self {
            ResponseBody::Text(text) => Some(text.as_str()),
            ResponseBody::None => Some(""),
            ResponseBody::Binary(_) => None,
        }
    }

    /// shorthand to reference body data as a byte slice.
    pub fn data(&self) -> &[u8] {
        match self {
            ResponseBody::None => &[],
            ResponseBody::Binary(data) => data.as_slice(),
            ResponseBody::Text(text) => text.as_bytes(),
        }
    }

    /// byte-size of body data.
    pub fn len(&self) -> usize {
        self.data().len()
    }

    /// a body with no data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
