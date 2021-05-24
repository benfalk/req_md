use std::ops::Range;
use crate::application::TimeoutDuration;

#[derive(Debug)]
pub struct Meta {
    pub line_range: Range<u32>,
    pub timeout: Option<TimeoutDuration>,
}
