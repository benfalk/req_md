use crate::id::Id;
use std::fmt::Debug;

/// # Entity Trait
///
/// An entity is a core concept in ReqMD representing a distinct
/// object with a unique identifier.  All entities must implement
/// this trait in order to be managed within the system.
///
pub trait Entity: Clone + Debug {
    fn id(&self) -> Id;
}
