use super::Point;
use ::markdown::unist;

/// Source code position with start and end points.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Position {
    pub start: Point,
    pub end: Point,
}

impl Position {
    /// Extends the current position to include another position.
    pub fn extend(&mut self, other: &Position) {
        if other.start < self.start {
            self.start = other.start.clone();
        }
        if other.end > self.end {
            self.end = other.end.clone();
        }
    }
}

impl From<unist::Position> for Position {
    fn from(value: unist::Position) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<&unist::Position> for Position {
    fn from(value: &unist::Position) -> Self {
        Self {
            start: Point::from(&value.start),
            end: Point::from(&value.end),
        }
    }
}
