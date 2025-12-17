//!
//! # ReqMD Core
//!
//#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

pub use error::Error;
pub use file::File;

mod error;
mod file;

#[cfg(test)]
mod support;
