//!
//! # ReqMD Core
//!

pub use error::Error;
pub use factory::{DefaultProvider, Factory, FactoryProcessor};
pub use file::File;
pub use md_request::MdRequest;

pub mod builtin_processors;
pub mod builtin_providers;

mod error;
mod factory;
mod file;
mod md_request;

#[cfg(test)]
mod support;
