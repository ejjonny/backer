use crate::layout::{NodeValue, Scopable};

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
#[derive(Debug)]
pub struct Node<State: Scopable> {
    pub(crate) inner: NodeValue<State>,
}

impl<State: Scopable> Clone for Node<State> {
    fn clone(&self) -> Self {
        Node {
            inner: self.inner.clone(),
        }
    }
}
