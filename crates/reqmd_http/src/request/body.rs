/// # Body Content
///
/// In HTTP messages, the content describes the 'information' conveyed
/// in the message body (which follows the header section), after any
/// message framing from HTTP/1.1 chunked transfer encoding has been
/// removed. This was referred to as a "payload" in HTTP/1.1, but
/// message "content" distinguishes from frame payloads in HTTP/2 and
/// HTTP/3 where the data in a single frame could be header data,
/// body data, or other control information.
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequestBody {
    /// No body is present in the request.
    #[default]
    None,

    /// Body is a binary representation of data.
    Binary(Vec<u8>),

    /// Body is a UTF-8 encoded text.
    Text(String),
}

impl RequestBody {
    /// shorthand to reference body text if it is UTF-8 encoded.
    pub fn text(&self) -> Option<&str> {
        match self {
            RequestBody::Text(text) => Some(text.as_str()),
            _ => None,
        }
    }

    /// shorthand to reference body data as a byte slice.
    pub fn data(&self) -> &[u8] {
        match self {
            RequestBody::None => &[],
            RequestBody::Binary(data) => data.as_slice(),
            RequestBody::Text(text) => text.as_bytes(),
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
