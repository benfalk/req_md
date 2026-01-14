use super::Point;
use crate::Error;
use ::markdown::{mdast, unist};
use ::std::ops::Range;

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

    // returns the substring based on the start and end offsets
    pub fn find_substring<'a>(&self, string: &'a str) -> Result<&'a str, Error> {
        let Some(substring) = string.get(self.start.offset..self.end.offset) else {
            return Err(Error::InvalidOffset {
                data: string.into(),
                position: Box::new(self.clone()),
            });
        };

        Ok(substring)
    }

    /// determines the range between this position and another position,
    /// if they do not overlap
    pub fn range_between(&self, other: &Self) -> Option<Range<usize>> {
        self.exclusive_with(other).then(|| {
            if self.end < other.start {
                self.end.offset..other.start.offset
            } else {
                other.end.offset..self.start.offset
            }
        })
    }

    /// determines if this position does not overlap with another position
    pub fn exclusive_with(&self, other: &Self) -> bool {
        self.start > other.end || self.end < other.start
    }

    /// determines if the position contains the specified line number
    pub fn contains_line(&self, line: usize) -> bool {
        self.start.line <= line && self.end.line >= line
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

impl TryFrom<&mdast::Code> for Position {
    type Error = Error;

    fn try_from(value: &mdast::Code) -> Result<Self, Self::Error> {
        let Some(position) = value.position.as_ref() else {
            return Err(Error::MissingPosition {
                node: Box::new(mdast::Node::Code(value.clone())),
            });
        };

        Ok(position.into())
    }
}

impl TryFrom<&mdast::Root> for Position {
    type Error = Error;

    fn try_from(value: &mdast::Root) -> Result<Self, Self::Error> {
        let Some(position) = value.position.as_ref() else {
            return Err(Error::MissingPosition {
                node: Box::new(mdast::Node::Root(value.clone())),
            });
        };

        Ok(position.into())
    }
}

impl TryFrom<&mdast::Heading> for Position {
    type Error = Error;

    fn try_from(value: &mdast::Heading) -> Result<Self, Self::Error> {
        let Some(position) = value.position.as_ref() else {
            return Err(Error::MissingPosition {
                node: Box::new(mdast::Node::Heading(value.clone())),
            });
        };

        Ok(position.into())
    }
}
