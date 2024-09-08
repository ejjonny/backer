use crate::{anynode::AnyNode, drawable::Drawable, models::*};

/**
The root object used to store & calculate a layout

# Quick Start

```rust

use backer::*;
use backer::models::*;
use backer::nodes::*;

let layout = Layout::new(my_layout_fn);

// UI libraries generally will expose methods to get the available screen size
// In a real implementation this should use the real screen size!
let available_area = Area {
        x: 0.,
        y: 0.,
        width: 100.,
        height: 100.,
    };
let mut my_state = MyState {};

let layout = Layout::new(my_layout_fn);
// Perform layout & draw all of your drawable nodes.
layout.draw(available_area, &mut my_state);

fn my_layout_fn(state: &mut MyState) -> Node<MyState> {
    // Your layout here
    row(vec![
        space(),
    ])
}
struct MyState {}
```
 */
pub struct Layout<State> {
    tree: fn(&mut State) -> Node<State>,
}

impl<State> Layout<State> {
    /// Creates a new [`Layout<State>`].
    pub fn new(tree: fn(&mut State) -> Node<State>) -> Self {
        Self { tree }
    }
}

impl<State> Layout<State> {
    /// Calculates layout and draws all draw nodes in the tree
    pub fn draw(&self, area: Area, state: &mut State) {
        let mut layout = (self.tree)(state);
        layout.inner.layout(area);
        layout.inner.draw(state);
    }
}

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

pub(crate) enum NodeValue<State> {
    Padding {
        amounts: Padding,
        element: Box<NodeValue<State>>,
    },
    Column {
        elements: Vec<NodeValue<State>>,
        spacing: f32,
    },
    Row {
        elements: Vec<NodeValue<State>>,
        spacing: f32,
    },
    Stack(Vec<NodeValue<State>>),
    Group(Vec<NodeValue<State>>),
    Offset {
        offset_x: f32,
        offset_y: f32,
        element: Box<NodeValue<State>>,
    },
    Draw(Drawable<State>),
    Explicit {
        options: Size,
        element: Box<NodeValue<State>>,
    },
    Empty,
    Space,
    Scope {
        scoped: AnyNode<State>,
    },
}

impl<State> NodeValue<State> {
    pub(crate) fn draw(&self, state: &mut State) {
        match self {
            NodeValue::Draw(drawable) => drawable.draw(drawable.area, state),
            NodeValue::Padding { element, .. }
            | NodeValue::Explicit { element, .. }
            | NodeValue::Offset { element, .. } => {
                element.draw(state);
            }
            NodeValue::Stack(elements) => {
                elements.iter().for_each(|el| el.draw(state));
            }
            NodeValue::Column { elements, .. } | NodeValue::Row { elements, .. } => {
                elements.iter().rev().for_each(|el| el.draw(state));
            }
            NodeValue::Space => (),
            NodeValue::Scope { scoped } => scoped.draw(state),
            NodeValue::Group(_) | NodeValue::Empty => unreachable!(),
        }
    }

    pub(crate) fn layout(&mut self, available_area: Area) {
        match self {
            NodeValue::Padding {
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
            NodeValue::Column { elements, spacing } => {
                layout_axis(elements, spacing, available_area, Orientation::Vertical)
            }
            NodeValue::Row { elements, spacing } => {
                layout_axis(elements, spacing, available_area, Orientation::Horizontal)
            }
            NodeValue::Stack(children) => {
                for child in children {
                    child.layout(available_area)
                }
            }
            NodeValue::Draw(drawable) => {
                drawable.area = Area {
                    x: available_area.x,
                    y: available_area.y,
                    width: available_area.width.max(0.),
                    height: available_area.height.max(0.),
                };
            }
            NodeValue::Explicit {
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
            NodeValue::Offset {
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
            NodeValue::Space => (),
            NodeValue::Scope { scoped } => scoped.layout(available_area),
            NodeValue::Group(_) | NodeValue::Empty => unreachable!(),
        }
    }
}

enum Orientation {
    Horizontal,
    Vertical,
}

fn layout_axis<State>(
    elements: &mut [NodeValue<State>],
    spacing: &f32,
    available_area: Area,
    orientation: Orientation,
) {
    let sizes: Vec<Option<f32>> = elements
        .iter_mut()
        .map(|element| match element {
            NodeValue::Explicit {
                options,
                element: _,
            } => match orientation {
                Orientation::Horizontal => {
                    if let Some(width) = options.width {
                        if options.x_relative {
                            options.width = None;
                            return Some(width * available_area.width);
                        } else {
                            return Some(width);
                        }
                    }
                    None
                }
                Orientation::Vertical => {
                    if let Some(height) = options.height {
                        if options.y_relative {
                            options.height = None;
                            return Some(height * available_area.height);
                        } else {
                            return Some(height);
                        }
                    }
                    None
                }
            },
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

    let element_count = sizes.len();

    let total_spacing = *spacing * (element_count as i32 - 1).max(0) as f32;
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

        current_pos += child_size + *spacing;
    }
}
