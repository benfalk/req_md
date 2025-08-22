#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Status(pub u16);

impl Default for Status {
    fn default() -> Self {
        Status(200) // Default to HTTP 200 OK
    }
}
