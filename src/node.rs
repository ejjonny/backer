use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
#[derive(Debug)]
pub struct Node<A, B> {
    pub(crate) inner: NodeValue<A, B>,
}

impl<A, B> Clone for Node<A, B> {
    fn clone(&self) -> Self {
        Node {
            inner: self.inner.clone(),
        }
    }
}
