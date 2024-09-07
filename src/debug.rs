use crate::layout::Node;
use std::fmt;

impl<State> fmt::Debug for Node<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Padding { amounts, element } => f
                .debug_struct("Padding")
                .field("amounts", amounts)
                .field("element", element)
                .finish(),
            Node::Column { elements, spacing } => f
                .debug_struct("Column")
                .field("elements", elements)
                .field("spacing", spacing)
                .finish(),
            Node::Row { elements, spacing } => f
                .debug_struct("Row")
                .field("elements", elements)
                .field("spacing", spacing)
                .finish(),
            Node::Stack(elements) => f.debug_tuple("Stack").field(elements).finish(),
            Node::Group(elements) => f.debug_tuple("Group").field(elements).finish(),
            Node::Offset {
                offset_x,
                offset_y,
                element,
            } => f
                .debug_struct("Offset")
                .field("offset_x", offset_x)
                .field("offset_y", offset_y)
                .field("element", element)
                .finish(),
            Node::Draw(drawable) => f.debug_tuple("Draw").field(drawable).finish(),
            Node::Explicit { options, element } => f
                .debug_struct("Explicit")
                .field("options", options)
                .field("element", element)
                .finish(),
            Node::Space => write!(f, "Space"),
            Node::Empty => write!(f, "Empty"),
            Node::Scope { scoped } => f.debug_struct("Scope").field("scoped", scoped).finish(),
        }
    }
}
