use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
#[derive(Debug)]
pub struct Node<State> {
    pub(crate) inner: NodeValue<State>,
}

impl<State> Clone for Node<State> {
    fn clone(&self) -> Self {
        Node {
            inner: self.inner.clone(),
        }
    }
}
