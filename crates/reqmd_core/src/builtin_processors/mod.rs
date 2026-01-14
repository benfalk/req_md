//! # Built-in Processors
//!
//! This module contains built-in [processors] that can be used to [modify]
//! [requests] during their creation in the [factory build] process.
//!
//! [processors]: crate::factory::FactoryProcessor
//! [modify]: crate::factory::FactoryProcessor::update_request
//! [requests]: crate::md_request::MdRequest
//! [factory build]: crate::factory::Factory::build_requests
//! ---

pub use env_var_expansion::EnvVarExpansion;
#[cfg(feature = "yaml-as-json")]
pub use yaml_as_json::YamlAsJson;

mod env_var_expansion;
#[cfg(feature = "yaml-as-json")]
mod yaml_as_json;
