use ::markdown::unist;

/// Defines a position in the source code where
/// data is located.  This includes line number,
/// column number, and byte offset.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Point {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl From<unist::Point> for Point {
    fn from(value: unist::Point) -> Self {
        Self {
            line: value.line,
            column: value.column,
            offset: value.offset,
        }
    }
}

impl From<&unist::Point> for Point {
    fn from(value: &unist::Point) -> Self {
        Self {
            line: value.line,
            column: value.column,
            offset: value.offset,
        }
    }
}
