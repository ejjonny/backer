use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub type Node<State> = NodeWith<State, ()>;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct NodeWith<State, Ctx> {
    pub(crate) inner: NodeValue<State, Ctx>,
}

impl<State, Ctx> Debug for NodeWith<State, Ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}
