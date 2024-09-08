use crate::layout::NodeValue;
use std::fmt;

impl<State> fmt::Debug for NodeValue<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeValue::Padding { amounts, element } => f
                .debug_struct("Padding")
                .field("amounts", amounts)
                .field("element", element)
                .finish(),
            NodeValue::Column { elements, spacing } => f
                .debug_struct("Column")
                .field("elements", elements)
                .field("spacing", spacing)
                .finish(),
            NodeValue::Row { elements, spacing } => f
                .debug_struct("Row")
                .field("elements", elements)
                .field("spacing", spacing)
                .finish(),
            NodeValue::Stack(elements) => f.debug_tuple("Stack").field(elements).finish(),
            NodeValue::Group(elements) => f.debug_tuple("Group").field(elements).finish(),
            NodeValue::Offset {
                offset_x,
                offset_y,
                element,
            } => f
                .debug_struct("Offset")
                .field("offset_x", offset_x)
                .field("offset_y", offset_y)
                .field("element", element)
                .finish(),
            NodeValue::Draw(drawable) => f.debug_tuple("Draw").field(drawable).finish(),
            NodeValue::Explicit { options, element } => f
                .debug_struct("Explicit")
                .field("options", options)
                .field("element", element)
                .finish(),
            NodeValue::Space => write!(f, "Space"),
            NodeValue::Empty => write!(f, "Empty"),
            NodeValue::Scope { scoped } => f.debug_struct("Scope").field("scoped", scoped).finish(),
            NodeValue::Wrapping {
                axis,
                elements,
                axis_spacing,
                off_axis_spacing,
            } => todo!(),
        }
    }
}
