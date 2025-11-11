//!
//! # ReqMD Markdown
//!
pub use error::Error;
pub use parsing::parse_markdown;
pub mod ast;

mod error;
mod parsing;

#[cfg(test)]
mod support;
