#![warn(missing_docs)]

/*! A library for straight-forward UI layout.

Dependency free & framework-agnostic. Backer can be used in an index-based layout approach or with inline drawing code.

_This library **only** implements layout & could be integrated with a range of UI crates._
*/

mod anynode;
mod clone;
mod debug;
mod drawable;
mod layout;

/// Core objects
pub use layout::{Layout, Node};

/// Structs involved in layout definitions
pub mod models;

/// Builder-style node layout modifications
pub mod modifiers;

/// Layout core node construction
pub mod nodes;
