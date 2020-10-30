use std::ops::Range;

#[derive(Debug)]
pub struct Meta {
    pub line_range: Option<Range<u32>>,
}
