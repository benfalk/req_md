//! # Built-in Providers
//!
//! This module contains [providers] that supply the [default values]
//! used when creating [requests] via the [factory build] process.
//!
//! [providers]: crate::factory::DefaultProvider
//! [default values]: crate::factory::DefaultProvider::apply_global_defaults
//! [requests]: crate::md_request::MdRequest
//! [factory build]: crate::factory::Factory::build_requests
//! ---

pub use env_provider::EnvProvider;

mod env_provider;
