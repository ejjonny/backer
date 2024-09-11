use core::f32;

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

    pub(crate) fn sizes(&mut self) -> SizeConstraints {
        match self {
            NodeValue::Padding { amounts, element } => {
                element.sizes().accumulate(SizeConstraints {
                    width: Constraint {
                        lower: Some(amounts.trailing + amounts.leading),
                        upper: None,
                    },
                    height: Constraint {
                        lower: Some(amounts.bottom + amounts.top),
                        upper: None,
                    },
                })
            }
            NodeValue::Column { elements, .. } => elements.iter_mut().fold(
                SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                },
                |current, element| SizeConstraints {
                    width: current.width.combine(element.sizes().width),
                    height: current.height.accumulate(element.sizes().height),
                },
            ),
            NodeValue::Row { elements, .. } => elements.iter_mut().fold(
                SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                },
                |current, element| SizeConstraints {
                    width: current.width.accumulate(element.sizes().width),
                    height: current.height.combine(element.sizes().height),
                },
            ),
            NodeValue::Stack(elements) => {
                let cumulative_size = SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                };
                for element in elements {
                    cumulative_size.combine(element.sizes());
                }
                cumulative_size
            }
            NodeValue::Explicit { options, element } => {
                element.sizes().combine(SizeConstraints::from(*options))
            }
            NodeValue::Offset { .. } => {
                todo!()
            }
            NodeValue::Scope { .. } => todo!(),
            _ => SizeConstraints {
                width: Constraint::none(),
                height: Constraint::none(),
            },
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

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SizeConstraints {
    width: Constraint,
    height: Constraint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Constraint {
    lower: Option<f32>,
    upper: Option<f32>,
}

impl Constraint {
    fn none() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }
}

impl SizeConstraints {
    fn combine(self, other: Self) -> Self {
        SizeConstraints {
            width: self.width.combine(other.width),
            height: self.height.combine(other.height),
        }
    }
    fn accumulate(self, other: Self) -> Self {
        SizeConstraints {
            width: self.width.accumulate(other.width),
            height: self.height.accumulate(other.height),
        }
    }
}

impl Constraint {
    fn combine(self, other: Self) -> Self {
        // This always takes the bigger bound
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        // In terms of upper constraints - no constraint is biggest
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        Constraint { lower, upper }
    }
    fn accumulate(self, other: Self) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(bound)) | (Some(bound), None) => Some(bound),
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b),
        };
        Constraint { lower, upper }
    }
}

impl From<Size> for SizeConstraints {
    fn from(value: Size) -> Self {
        SizeConstraints {
            width: if value.width.is_some() {
                Constraint {
                    lower: value.width,
                    upper: value.width,
                }
            } else if value.width_min.is_some() || value.width_max.is_some() {
                Constraint {
                    lower: value.width_min,
                    upper: value.width_max,
                }
            } else {
                Constraint {
                    lower: None,
                    upper: None,
                }
            },
            height: if value.height.is_some() {
                Constraint {
                    lower: value.height,
                    upper: value.height,
                }
            } else if value.height_min.is_some() || value.height_max.is_some() {
                Constraint {
                    lower: value.height_min,
                    upper: value.height_max,
                }
            } else {
                Constraint {
                    lower: None,
                    upper: None,
                }
            },
        }
    }
}

fn layout_axis<State>(
    elements: &mut [NodeValue<State>],
    spacing: &f32,
    available_area: Area,
    orientation: Orientation,
) {
    let sizes: Vec<SizeConstraints> = elements.iter_mut().map(|element| element.sizes()).collect();
    let element_count = elements.len();

    let total_spacing = *spacing * (element_count as i32 - 1).max(0) as f32;
    let available_size = match orientation {
        Orientation::Horizontal => available_area.width,
        Orientation::Vertical => available_area.height,
    } - total_spacing;

    let default_size = available_size / element_count as f32;

    let mut pool = 0.;
    let mut final_sizes: Vec<Option<f32>> = elements.iter().map(|_| Option::<f32>::None).collect();
    let mut positive_room: Vec<f32> = elements.iter().map(|_| 0.).collect();
    let mut room_to_shrink: Vec<f32> = elements.iter().map(|_| 0.).collect();

    for (i, constraint) in sizes.iter().enumerate() {
        let constraint = match orientation {
            Orientation::Horizontal => constraint.width,
            Orientation::Vertical => constraint.height,
        };
        let Constraint { lower, upper } = constraint;
        if let Some(lower) = lower {
            if default_size < lower {
                pool += default_size - lower;
                final_sizes[i] = lower.into();
                continue;
            } else {
                room_to_shrink[i] = -(default_size - lower);
            }
        } else {
            // Effectively, this means the element can shrink to 0
            room_to_shrink[i] = -default_size;
        }
        if let Some(upper) = upper {
            if default_size > upper {
                pool += default_size - upper;
                final_sizes[i] = upper.into();
                continue;
            } else {
                positive_room[i] = -(default_size - upper);
            }
        } else {
            // Effectively, this means the element can expand any amount
            positive_room[i] = default_size * 2.;
        }

        final_sizes[i] = default_size.into();
    }

    fn room_available(room: &[f32]) -> bool {
        room.iter().filter(|r| r.abs() > 0.).count() as f32 > 0.
    }

    let limit = 4;
    let mut i = 0;
    loop {
        if i > limit {
            break;
        }
        i += 1;
        let pool_empty = pool.abs() < 0.1;
        if !pool_empty && pool.is_sign_positive() && room_available(&positive_room) {
            // We need to use more room
            let mut enumerated_room: Vec<(usize, f32)> = positive_room
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v))
                .filter(|(_, v)| *v != 0.)
                .collect();
            enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let distribution_candidates = positive_room
                .iter()
                .filter(|r| r.abs() > 0. && r.is_sign_positive())
                .count() as f32;
            let distribution_amount =
                (pool / distribution_candidates).min(enumerated_room.first().unwrap().1);
            pool -= distribution_amount * distribution_candidates;
            enumerated_room.iter().for_each(|&(i, _)| {
                if positive_room[i].abs() > 0. && positive_room[i].is_sign_positive() {
                    positive_room[i] -= distribution_amount;
                    if let Some(size) = &mut final_sizes[i] {
                        *size += distribution_amount
                    }
                }
            });
        } else if !pool_empty && pool.is_sign_negative() && room_available(&room_to_shrink) {
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
        Orientation::Horizontal => available_area.x,
        Orientation::Vertical => available_area.y,
    };
    for (i, child) in elements.iter_mut().enumerate() {
        let child_size = final_sizes[i].unwrap();
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

#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::nodes::*;
    #[test]
    fn test_column_basic() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 50.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 50., 100., 50.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_column_constrained_1() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 90.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 90.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_column_constrained_2() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 90.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 100., 10.));
                })
                .height(10.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 90.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 100., 10.));
                })
                .height(10.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_basic() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_constrained_1() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 25., 10., 50.));
                })
                .width(10.)
                .height(50.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 0., 90., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .y_align(YAlign::Top),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 40., 10., 20.));
                })
                .width(10.)
                .height(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(20., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .y_align(YAlign::Bottom),
                draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 70., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_constrained_2() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 70., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .y_align(YAlign::Top),
                draw(|a, _| {
                    assert_eq!(a, Area::new(80., 40., 10., 20.));
                })
                .width(10.)
                .height(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .y_align(YAlign::Bottom),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 70., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .y_align(YAlign::Top),
                draw(|a, _| {
                    assert_eq!(a, Area::new(80., 40., 10., 20.));
                })
                .width(10.)
                .height(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .y_align(YAlign::Bottom),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_stack_basic() {
        Layout::new(|()| {
            stack(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_stack_alignment() {
        Layout::new(|()| {
            stack(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::TopLeading),
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::TopCenter),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::TopTrailing),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 40., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::CenterTrailing),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::BottomTrailing),
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::BottomCenter),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::BottomLeading),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::CenterLeading),
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 40., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::CenterCenter),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_sequence_spacing() {
        Layout::new(|()| {
            row_spaced(
                10.,
                vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(20., 0., 25., 100.));
                    }),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(55., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(75., 0., 25., 100.));
                    }),
                ],
            )
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column_spaced(
                10.,
                vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 15.));
                    }),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(45., 25., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 55., 100., 15.));
                    }),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(45., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                ],
            )
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_with_constrained_item() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 30., 100.));
                })
                .width(30.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 70., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_nested_row_with_constrained_item() {
        Layout::new(|()| {
            row(vec![
                row(vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 0., 20., 100.));
                    })
                    .width(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(20., 0., 30., 100.));
                    }),
                ])
                .width(50.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_stack_with_constrained_item() {
        Layout::new(|()| {
            stack(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(25., 25., 50., 50.));
                })
                .width(50.)
                .height(50.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_row_with_multiple_constrained_items() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 20., 100.));
                })
                .width(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(20., 25., 30., 50.));
                })
                .width(30.)
                .height(50.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 25., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(75., 0., 25., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_constraint_combination() {
        assert_eq!(
            row::<()>(vec![space(), space().height(30.)]).inner.sizes(),
            SizeConstraints {
                width: Constraint::none(),
                height: Constraint {
                    lower: Some(30.),
                    upper: None
                }
            }
        );
        assert_eq!(
            row::<()>(vec![space().height(40.), space().height(30.)])
                .inner
                .sizes(),
            SizeConstraints {
                width: Constraint::none(),
                height: Constraint {
                    lower: Some(40.),
                    upper: None
                }
            }
        );
        assert_eq!(
            row::<()>(vec![space().height(40.), space().height_range(30.0..35.)])
                .inner
                .sizes(),
            SizeConstraints {
                width: Constraint::none(),
                height: Constraint {
                    lower: Some(40.),
                    upper: None
                }
            }
        );
        assert_eq!(
            column::<()>(vec![space().width(20.).height(50.), space().width(10.)])
                .inner
                .sizes(),
            SizeConstraints {
                width: Constraint {
                    lower: Some(20.),
                    upper: None
                },
                height: Constraint {
                    lower: Some(50.),
                    upper: None
                }
            }
        );
    }
}
