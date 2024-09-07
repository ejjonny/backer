use crate::{anynode::AnyNode, drawable::Drawable, models::*};

pub struct Layout<State> {
    pub tree: fn(&State) -> Node<State>,
}

impl<State> Layout<State> {
    pub fn draw(&self, area: Area, state: &mut State) {
        let mut layout = (self.tree)(state);
        layout.layout(area);
        layout.draw(state);
    }
}

pub enum Node<State> {
    Padding {
        amounts: Padding,
        element: Box<Node<State>>,
    },
    Column {
        elements: Vec<Node<State>>,
        spacing: f32,
    },
    Row {
        elements: Vec<Node<State>>,
        spacing: f32,
    },
    Stack(Vec<Node<State>>),
    Group(Vec<Node<State>>),
    Offset {
        offset_x: f32,
        offset_y: f32,
        element: Box<Node<State>>,
    },
    Draw(Drawable<State>),
    Explicit {
        options: Size,
        element: Box<Node<State>>,
    },
    Conditional {
        condition: bool,
        element: Box<Node<State>>,
    },
    Space,
    Scope {
        scoped: AnyNode<State>,
    },
}

impl<State> Node<State> {
    pub fn draw(&self, state: &mut State) {
        match self {
            Node::Draw(drawable) => drawable.draw(drawable.area, state),
            Node::Padding { element, .. }
            | Node::Explicit { element, .. }
            | Node::Offset { element, .. }
            | Node::Conditional { element, .. } => {
                element.draw(state);
            }
            Node::Column { elements, .. } | Node::Row { elements, .. } | Node::Stack(elements) => {
                elements.iter().rev().for_each(|el| el.draw(state));
            }
            Node::Space => (),
            Node::Scope { scoped } => scoped.draw(state),
            Node::Group(_) => unreachable!(),
        }
    }

    pub fn layout(&mut self, available_area: Area) {
        match self {
            Node::Padding {
                amounts,
                element: child,
            } => {
                let inner_area = Area {
                    x: available_area.x + amounts.leading,
                    y: available_area.y + amounts.top,
                    width: available_area.width - amounts.trailing - amounts.leading,
                    height: available_area.height - amounts.bottom - amounts.top,
                };
                child.layout(inner_area);
            }
            Node::Column { elements, spacing } => {
                layout_axis(elements, spacing, available_area, Orientation::Vertical)
            }
            Node::Row { elements, spacing } => {
                layout_axis(elements, spacing, available_area, Orientation::Horizontal)
            }
            Node::Stack(children) => {
                for child in children {
                    child.layout(available_area)
                }
            }
            Node::Draw(drawable) => {
                drawable.area = Area {
                    x: available_area.x,
                    y: available_area.y,
                    width: available_area.width.max(0.),
                    height: available_area.height.max(0.),
                };
            }
            Node::Explicit {
                options,
                element: child,
            } => {
                let Size {
                    width,
                    width_min,
                    width_max,
                    height,
                    height_min,
                    height_max,
                    x_align,
                    y_align,
                    x_relative,
                    y_relative,
                } = options;
                let explicit_width = if *x_relative {
                    available_area.width * width.unwrap_or(1.0)
                } else {
                    width.unwrap_or(available_area.width)
                }
                .clamp(
                    width_min.unwrap_or(0.).min(width_max.unwrap_or(0.)),
                    width_max
                        .unwrap_or(available_area.width.max(0.))
                        .max(width_min.unwrap_or(0.)),
                );
                let explicit_height = if *y_relative {
                    available_area.height * height.unwrap_or(1.0)
                } else {
                    height.unwrap_or(available_area.height)
                }
                .clamp(
                    height_min.unwrap_or(0.).min(height_max.unwrap_or(0.)),
                    height_max
                        .unwrap_or(available_area.height.max(0.))
                        .max(height_min.unwrap_or(0.)),
                );
                let x = match x_align {
                    XAlign::Leading => available_area.x,
                    XAlign::Trailing => available_area.x + (available_area.width - explicit_width),
                    XAlign::Center => {
                        available_area.x + (available_area.width * 0.5) - (explicit_width * 0.5)
                    }
                };
                let y = match y_align {
                    YAlign::Top => available_area.y,
                    YAlign::Bottom => available_area.y + (available_area.height - explicit_height),
                    YAlign::Center => {
                        available_area.y + (available_area.height * 0.5) - (explicit_height * 0.5)
                    }
                };
                child.layout(Area {
                    x: x.max(available_area.x),
                    y: y.max(available_area.y),
                    width: explicit_width,
                    height: explicit_height,
                });
            }
            Node::Offset {
                offset_x,
                offset_y,
                element,
            } => {
                element.layout(Area {
                    x: available_area.x + *offset_x,
                    y: available_area.y + *offset_y,
                    width: available_area.width,
                    height: available_area.height,
                });
            }
            Node::Conditional {
                condition: _,
                element,
            } => element.layout(available_area),
            Node::Space => (),
            Node::Scope { scoped } => scoped.layout(available_area),
            Node::Group(_) => unreachable!(),
        }
    }
}

enum Orientation {
    Horizontal,
    Vertical,
}

fn layout_axis<State>(
    elements: &mut [Node<State>],
    spacing: &f32,
    available_area: Area,
    orientation: Orientation,
) {
    let sizes: Vec<Option<f32>> = elements
        .iter()
        .map(|element| match element {
            Node::Conditional {
                condition,
                element: _,
            } => {
                if *condition {
                    None
                } else {
                    Some(0.)
                }
            }
            Node::Explicit {
                options,
                element: _,
            } => {
                if (matches!(orientation, Orientation::Horizontal) && options.x_relative)
                    || (matches!(orientation, Orientation::Vertical) && options.y_relative)
                {
                    None
                } else {
                    match orientation {
                        Orientation::Horizontal => options.width,
                        Orientation::Vertical => options.height,
                    }
                }
            }
            _ => None,
        })
        .map(|size| {
            let Some(size) = size else { return size };
            match orientation {
                Orientation::Horizontal => Some(size.min(available_area.width)),
                Orientation::Vertical => Some(size.min(available_area.height)),
            }
        })
        .collect();

    let non_zero_elements = sizes
        .iter()
        .filter(|&s| s.map_or(true, |v| v != 0.))
        .count();

    let total_spacing = *spacing * (non_zero_elements as i32 - 1).max(0) as f32;
    let available_size = match orientation {
        Orientation::Horizontal => available_area.width,
        Orientation::Vertical => available_area.height,
    } - total_spacing;

    let explicit_consumed = sizes.iter().filter_map(|&s| s).sum::<f32>();
    let remaining = available_size - explicit_consumed;
    let unconstrained_sizes = sizes.iter().filter(|&s| s.is_none()).count();
    let default_size = remaining / unconstrained_sizes as f32;

    let mut current_pos = match orientation {
        Orientation::Horizontal => available_area.x,
        Orientation::Vertical => available_area.y,
    };

    for (i, child) in elements.iter_mut().enumerate() {
        let child_size = sizes
            .get(i)
            .unwrap_or(&Some(default_size))
            .unwrap_or(default_size);

        let area = match orientation {
            Orientation::Horizontal => Area {
                x: current_pos,
                y: available_area.y,
                width: child_size,
                height: available_area.height,
            },
            Orientation::Vertical => Area {
                x: available_area.x,
                y: current_pos,
                width: available_area.width,
                height: child_size,
            },
        };

        child.layout(area);

        if child_size > 0. {
            current_pos += child_size + *spacing;
        }
    }
}
