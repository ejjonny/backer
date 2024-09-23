use crate::{drawable::Drawable, layout::NodeValue};

impl<State> Clone for NodeValue<State> {
    fn clone(&self) -> Self {
        match self {
            NodeValue::Padding { amounts, element } => NodeValue::Padding {
                amounts: *amounts,
                element: element.clone(),
            },
            NodeValue::Column {
                elements,
                spacing,
                align,
                off_axis_align,
            } => NodeValue::Column {
                elements: elements.clone(),
                spacing: *spacing,
                align: *align,
                off_axis_align: *off_axis_align,
            },
            NodeValue::Row {
                elements,
                spacing,
                align,
                off_axis_align,
            } => NodeValue::Row {
                elements: elements.clone(),
                spacing: *spacing,
                align: *align,
                off_axis_align: *off_axis_align,
            },
            NodeValue::Stack(elements) => NodeValue::Stack(elements.clone()),
            NodeValue::Offset {
                offset_x,
                offset_y,
                element,
            } => NodeValue::Offset {
                offset_x: *offset_x,
                offset_y: *offset_y,
                element: element.clone(),
            },
            NodeValue::Draw(drawable) => NodeValue::Draw(Drawable {
                area: drawable.area,
                draw: drawable.draw.clone(),
            }),
            NodeValue::Explicit { options, element } => NodeValue::Explicit {
                options: options.clone(),
                element: element.clone(),
            },
            NodeValue::Group(elements) => NodeValue::Group(elements.clone()),
            NodeValue::Space => NodeValue::Space,
            NodeValue::Scope { scoped } => NodeValue::Scope {
                scoped: scoped.clone(),
            },
            NodeValue::Empty => NodeValue::Empty,
            NodeValue::AreaReader { read } => NodeValue::AreaReader { read: read.clone() },
            NodeValue::Coupled {
                element,
                coupled,
                over,
            } => NodeValue::Coupled {
                element: element.clone(),
                coupled: coupled.clone(),
                over: *over,
            },
        }
    }
}
