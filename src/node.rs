use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub type Node<A> = NodeWith<A, ()>;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
#[derive(Debug)]
pub struct NodeWith<A, B> {
    pub(crate) inner: NodeValue<A, B>,
}

// impl<A, B> Clone for NodeWith<A, B> {
//     fn clone(&self) -> Self {
// NodeWith {
//             inner: self.inner.clone(),
//         }
//     }
// }
