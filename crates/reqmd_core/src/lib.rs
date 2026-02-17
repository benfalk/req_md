//!
//! # ReqMD Core
//!

pub use error::Error;
pub use factory::{DefaultProvider, Factory, FactoryProcessor};
pub use file::File;
#[cfg(feature = "http-client")]
pub use http_client::HttpClient;
pub use md_request::MdRequest;
pub use md_request_list::MdRequestList;

pub mod builtin_processors;
pub mod builtin_providers;
pub use ::reqmd_http as http;

mod error;
mod factory;
mod file;
#[cfg(feature = "http-client")]
mod http_client;
mod md_request;
mod md_request_list;

#[cfg(test)]
mod support;
