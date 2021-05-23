use std::ops::Range;

#[derive(Debug)]
pub struct Meta {
    pub line_range: Range<u32>,
    pub timeout: Option<usize>,
}
