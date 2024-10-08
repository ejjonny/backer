#![warn(missing_docs)]

/*!
A library for straight-forward UI layout.

Dependency free & framework-agnostic. Backer can be used in an index-based layout approach or with inline drawing code.

_This library **only** implements layout & could be integrated with a range of UI crates._

# Quick Start
See [`Layout`] for setup.

See [`Node`] for layout customization.
*/

mod anynode;
mod clone;
mod constraints;
mod debug;
mod drawable;
mod layout;
mod modifiers;
mod node;
mod tests;

pub use layout::Layout;
pub use node::Node;

/// Structs involved in layout definitions
pub mod models;

/// Layout core node construction
pub mod nodes;
