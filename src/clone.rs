use crate::{drawable::Drawable, layout::Node};

impl<State> Clone for Node<State> {
    fn clone(&self) -> Self {
        match self {
            Node::Padding { amounts, element } => Node::Padding {
                amounts: *amounts,
                element: element.clone(),
            },
            Node::Column { elements, spacing } => Node::Column {
                elements: elements.clone(),
                spacing: *spacing,
            },
            Node::Row { elements, spacing } => Node::Row {
                elements: elements.clone(),
                spacing: *spacing,
            },
            Node::Stack(elements) => Node::Stack(elements.clone()),
            Node::Offset {
                offset_x,
                offset_y,
                element,
            } => Node::Offset {
                offset_x: *offset_x,
                offset_y: *offset_y,
                element: element.clone(),
            },
            Node::Draw(drawable) => Node::Draw(Drawable {
                area: drawable.area,
                draw: drawable.draw.clone(),
            }),
            Node::Explicit { options, element } => Node::Explicit {
                options: *options,
                element: element.clone(),
            },
            Node::Group(elements) => Node::Group(elements.clone()),
            Node::Space => Node::Space,
            Node::Scope { scoped } => Node::Scope {
                scoped: scoped.clone(),
            },
            Node::Empty => Node::Empty,
        }
    }
}
