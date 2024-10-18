use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
// pub type Node<'a, State> = NodeWith<'a, State>;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<'a, State> {
    pub(crate) inner: NodeValue<'a, State>,
}

impl<'a, State> Debug for Node<'a, State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}
