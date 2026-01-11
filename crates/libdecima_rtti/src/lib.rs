//! Decima has a custom RTTI system used widely across the engine.
//!
//! This crate is broken into two main modules:
//! 1. [rtti] (todo) - which provides Rust safe versions of Decima's RTTI types, and
//! 2. [sys] - which provides direct ffi bindings for Decima's RTTI types.
//!  
//!

// todo(py): different games have different versions of the RTTI structs, so we should have toggles via cargo features

pub mod string;
pub mod sys;

pub mod prelude {
    pub use crate::RTTIWithAliases;
    pub use crate::RTTIWithName;
    pub use crate::RTTIWithValues;
    pub use crate::string::*;
    pub use crate::sys::*;
}

// todo(py): replace with rust safe rtti impls
pub trait RTTIWithAliases {
    fn aliases(&self) -> Vec<String>;
}
pub trait RTTIWithName {
    fn symbol_name(&self) -> String;
}
pub trait RTTIWithValues {
    type Value;
    fn values(&self) -> Vec<Self::Value>;
}
