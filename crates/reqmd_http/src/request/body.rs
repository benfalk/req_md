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
pub enum Body {
    /// No body is present in the request.
    #[default]
    None,

    /// Body is a binary representation of data.
    Binary(Vec<u8>),

    /// Body is a UTF-8 encoded text.
    Text(String),
}
