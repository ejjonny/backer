use crate::{
    anynode::AnyNode,
    constraints::{Constraint, SizeConstraints},
    drawable::Drawable,
    models::*,
    Node,
};
use core::f32;
use std::rc::Rc;

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
#[derive(Debug, Clone)]
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
        layout.inner.layout(area, None, None, state);
        layout.inner.draw(state);
    }
}

type AreaReaderFn<State> = Rc<dyn Fn(Area, &mut State) -> Node<State>>;

pub(crate) enum NodeValue<State> {
    Padding {
        amounts: Padding,
        element: Box<NodeValue<State>>,
    },
    Column {
        elements: Vec<NodeValue<State>>,
        spacing: f32,
        align: Option<YAlign>,
        off_axis_align: Option<XAlign>,
    },
    Row {
        elements: Vec<NodeValue<State>>,
        spacing: f32,
        align: Option<XAlign>,
        off_axis_align: Option<YAlign>,
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
    AreaReader {
        read: AreaReaderFn<State>,
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
            NodeValue::Group(_) | NodeValue::Empty | NodeValue::AreaReader { .. } => {
                unreachable!()
            }
        }
    }

    pub(crate) fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        match self {
            NodeValue::Padding {
                amounts,
                element: child,
            } => {
                let inner_area = Area {
                    x: available_area.x + amounts.leading,
                    y: available_area.y + amounts.top,
                    width: (available_area.width - amounts.trailing - amounts.leading).max(0.),
                    height: (available_area.height - amounts.bottom - amounts.top).max(0.),
                };
                child.layout(inner_area, None, None, state);
            }
            NodeValue::Column {
                elements,
                spacing,
                align,
                off_axis_align,
            } => layout_axis(
                elements,
                spacing,
                available_area,
                Orientation::Vertical,
                off_axis_align.unwrap_or(XAlign::Center),
                align.unwrap_or(YAlign::Center),
                state,
            ),
            NodeValue::Row {
                elements,
                spacing,
                align,
                off_axis_align,
            } => layout_axis(
                elements,
                spacing,
                available_area,
                Orientation::Horizontal,
                align.unwrap_or(XAlign::Center),
                off_axis_align.unwrap_or(YAlign::Center),
                state,
            ),
            NodeValue::Stack(children) => {
                for child in children {
                    child.layout(available_area, None, None, state)
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
                    x_align: explicit_x_align,
                    y_align: explicit_y_align,
                    ..
                } = *options;
                let x_align = explicit_x_align
                    .or(contextual_x_align)
                    .unwrap_or(XAlign::Center);
                let y_align = explicit_y_align
                    .or(contextual_y_align)
                    .unwrap_or(YAlign::Center);
                let available_area =
                    available_area.constrained(SizeConstraints::from(*options), x_align, y_align);
                child.layout(available_area, None, None, state);
            }
            NodeValue::Offset {
                offset_x,
                offset_y,
                element,
            } => {
                element.layout(
                    Area {
                        x: available_area.x + *offset_x,
                        y: available_area.y + *offset_y,
                        width: available_area.width,
                        height: available_area.height,
                    },
                    None,
                    None,
                    state,
                );
            }
            NodeValue::Space => (),
            NodeValue::Scope { scoped } => scoped.layout(available_area, state),
            NodeValue::AreaReader { read } => {
                *self = read(available_area, state).inner;
                self.layout(available_area, None, None, state);
            }
            NodeValue::Group(_) | NodeValue::Empty => unreachable!(),
        }
    }
}

impl Area {
    fn constrained(self, constraints: SizeConstraints, x_align: XAlign, y_align: YAlign) -> Self {
        let mut width = match (constraints.width.lower, constraints.width.upper) {
            (None, None) => self.width,
            (None, Some(upper)) => self.width.min(upper),
            (Some(lower), None) => self.width.max(lower),
            (Some(lower), Some(upper)) => self.width.clamp(lower, upper.max(lower)),
        };
        let mut height = match (constraints.height.lower, constraints.height.upper) {
            (None, None) => self.height,
            (None, Some(upper)) => self.height.min(upper),
            (Some(lower), None) => self.height.max(lower),
            (Some(lower), Some(upper)) => self.height.clamp(lower, upper.max(lower)),
        };
        if let Some(aspect) = constraints.aspect {
            width = (height * aspect).min(width);
            height = (width / aspect).min(height);
        }
        let x = match x_align {
            XAlign::Leading => self.x,
            XAlign::Trailing => self.x + (self.width - width),
            XAlign::Center => self.x + (self.width * 0.5) - (width * 0.5),
        };
        let y = match y_align {
            YAlign::Top => self.y,
            YAlign::Bottom => self.y + (self.height - height),
            YAlign::Center => self.y + (self.height * 0.5) - (height * 0.5),
        };
        Area {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn layout_axis<State>(
    elements: &mut [NodeValue<State>],
    spacing: &f32,
    available_area: Area,
    orientation: Orientation,
    x_align: XAlign,
    y_align: YAlign,
    state: &mut State,
) {
    let sizes: Vec<SizeConstraints> = elements
        .iter_mut()
        .map(|element| element.constraints(available_area))
        .collect();
    let element_count = elements.len();

    let total_spacing = *spacing * (element_count as i32 - 1).max(0) as f32;
    let available_size = match orientation {
        Orientation::Horizontal => available_area.width,
        Orientation::Vertical => available_area.height,
    } - total_spacing;

    let default_size = available_size / element_count as f32;

    let mut pool = 0.;
    let mut final_sizes: Vec<Option<f32>> = elements.iter().map(|_| Option::<f32>::None).collect();
    let mut room_to_grow: Vec<f32> = elements.iter().map(|_| 0.).collect();
    let mut room_to_shrink: Vec<f32> = elements.iter().map(|_| 0.).collect();

    for (i, size_constraint) in sizes.iter().enumerate() {
        let constraint = match orientation {
            Orientation::Horizontal => size_constraint.width,
            Orientation::Vertical => size_constraint.height,
        };
        let mut final_size = Option::<f32>::None;
        let Constraint {
            mut lower,
            mut upper,
        } = constraint;

        if let Some(aspect) = size_constraint.aspect {
            match orientation {
                Orientation::Horizontal => {
                    let value = sizes[i].height.clamping(available_area.height) * aspect;
                    lower = Some(value);
                    upper = Some(value);
                }
                Orientation::Vertical => {
                    let value = sizes[i].width.clamping(available_area.width) / aspect;
                    lower = Some(value);
                    upper = Some(value);
                }
            }
        }

        if let Some(lower) = lower {
            if default_size < lower {
                pool += default_size - lower;
                final_size = Some(lower);
            }
        }
        if let Some(upper) = upper {
            if default_size > upper {
                pool += default_size - upper;
                final_size = Some(upper);
            }
        }

        if let Some(lower) = lower {
            if default_size >= lower {
                room_to_shrink[i] = -(final_size.unwrap_or(default_size) - lower);
            }
        } else {
            // Effectively, this means the element can shrink to 0
            room_to_shrink[i] = -default_size;
        }

        if let Some(upper) = upper {
            if default_size <= upper {
                room_to_grow[i] = -(final_size.unwrap_or(default_size) - upper);
            }
        } else {
            // Effectively, this means the element can expand any amount
            room_to_grow[i] = default_size * 10.;
        }

        final_sizes[i] = final_size.unwrap_or(default_size).into();
    }

    fn can_accommodate(room: &[f32]) -> bool {
        room.iter().filter(|r| r.abs() > 0.).count() as f32 > 0.
    }

    let limit = 5;
    let mut i = 0;
    loop {
        if i > limit {
            break;
        }
        i += 1;
        let pool_empty = pool.abs() < 0.1;
        if !pool_empty && pool.is_sign_positive() && can_accommodate(&room_to_grow) {
            // We need to use more room
            let mut enumerated_room: Vec<(usize, f32)> = room_to_grow
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v))
                .filter(|(_, v)| *v != 0.)
                .collect();
            enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let distribution_candidates = room_to_grow
                .iter()
                .filter(|r| r.abs() > 0. && r.is_sign_positive())
                .count() as f32;
            let distribution_amount =
                (pool / distribution_candidates).min(enumerated_room.first().unwrap().1);
            pool -= distribution_amount * distribution_candidates;
            enumerated_room.iter().for_each(|&(i, _)| {
                if room_to_grow[i].abs() > 0. && room_to_grow[i].is_sign_positive() {
                    room_to_grow[i] -= distribution_amount;
                    if let Some(size) = &mut final_sizes[i] {
                        *size += distribution_amount
                    }
                }
            });
        } else if !pool_empty && pool.is_sign_negative() && can_accommodate(&room_to_shrink) {
            // We need to use less room
            let mut enumerated_room: Vec<(usize, f32)> = room_to_shrink
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v))
                .filter(|(_, v)| *v != 0.)
                .collect();
            enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
            let distribution_candidates = room_to_shrink
                .iter()
                .filter(|r| r.abs() > 0. && r.is_sign_negative())
                .count() as f32;
            let distribution_amount =
                (pool / distribution_candidates).max(enumerated_room.first().unwrap().1);
            pool -= distribution_amount * distribution_candidates;
            enumerated_room.iter().for_each(|&(i, _)| {
                if room_to_shrink[i].abs() > 0. && room_to_shrink[i].is_sign_negative() {
                    room_to_shrink[i] -= distribution_amount;
                    if let Some(size) = &mut final_sizes[i] {
                        *size += distribution_amount
                    }
                }
            });
        } else {
            break;
        }
    }

    let mut current_pos = match orientation {
        Orientation::Horizontal => match x_align {
            XAlign::Leading => available_area.x,
            XAlign::Center => available_area.x + (pool * 0.5),
            XAlign::Trailing => available_area.x + pool,
        },
        Orientation::Vertical => match y_align {
            YAlign::Top => available_area.y,
            YAlign::Center => available_area.y + (pool * 0.5),
            YAlign::Bottom => available_area.y + pool,
        },
    };

    for (i, child) in elements.iter_mut().enumerate() {
        let child_size = final_sizes[i].unwrap();

        let mut area = match orientation {
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

        if let NodeValue::Explicit { .. } = child {
        } else {
            area = area.constrained(sizes[i], x_align, y_align)
        }

        child.layout(area, Some(x_align), Some(y_align), state);

        current_pos += child_size + *spacing;
    }
}
