//!
//! # ReqMD Core
//!
//#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

pub use entity::Entity;
pub use entity_collection::EntityCollection;
pub use http_defaults::HttpDefaults;
pub use http_group::HttpGroup;
pub use http_inputs::HttpInputs;
pub use id::{Id, IdError};

mod entity;
mod entity_collection;
mod http_defaults;
mod http_group;
mod http_inputs;
mod id;
