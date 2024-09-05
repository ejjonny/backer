use std::rc::Rc;

use crate::models::*;

pub enum Layout<C> {
    Padding {
        amounts: Padding,
        element: Box<Layout<C>>,
    },
    Column {
        elements: Vec<Layout<C>>,
        spacing: f32,
    },
    Row {
        elements: Vec<Layout<C>>,
        spacing: f32,
    },
    Stack(Vec<Layout<C>>),
    Offset {
        offset_x: f32,
        offset_y: f32,
        element: Box<Layout<C>>,
    },
    Draw(Drawable<C>),
    Explicit {
        options: Size,
        element: Box<Layout<C>>,
    },
    Conditional {
        condition: bool,
        element: Box<Layout<C>>,
    },
}

impl<C> Clone for Layout<C> {
    fn clone(&self) -> Self {
        match self {
            Layout::Padding { amounts, element } => Layout::Padding {
                amounts: *amounts,
                element: element.clone(),
            },
            Layout::Column { elements, spacing } => Layout::Column {
                elements: elements.clone(),
                spacing: *spacing,
            },
            Layout::Row { elements, spacing } => Layout::Row {
                elements: elements.clone(),
                spacing: *spacing,
            },
            Layout::Stack(elements) => Layout::Stack(elements.clone()),
            Layout::Offset {
                offset_x,
                offset_y,
                element,
            } => Layout::Offset {
                offset_x: *offset_x,
                offset_y: *offset_y,
                element: element.clone(),
            },
            Layout::Draw(drawable) => Layout::Draw(Drawable {
                area: drawable.area,
                draw: drawable.draw.clone(),
            }),
            Layout::Explicit { options, element } => Layout::Explicit {
                options: *options,
                element: element.clone(),
            },
            Layout::Conditional { condition, element } => Layout::Conditional {
                condition: *condition,
                element: element.clone(),
            },
        }
    }
}

type DrawFn<Context> = Rc<dyn Fn(Area, &'_ mut Context)>;

#[derive(Clone)]
pub struct Drawable<Context> {
    pub area: Area,
    pub(crate) draw: DrawFn<Context>,
}

impl<Context> Drawable<Context> {
    pub fn draw(&self, area: Area, ctx: &mut Context) {
        if area.width > 0. && area.height > 0. {
            (self.draw)(area, ctx);
        }
    }
}

pub struct LayoutNodeIterator<'a, U> {
    stack: Vec<&'a Layout<U>>,
}

impl<'a, U> Iterator for LayoutNodeIterator<'a, U> {
    type Item = &'a Layout<U>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(layout) = self.stack.pop() {
            match layout {
                // Leaf
                Layout::Draw(_) => return Some(layout),
                // Group
                Layout::Column { elements, .. }
                | Layout::Row { elements, .. }
                | Layout::Stack(elements) => {
                    self.stack.extend(elements.iter().rev());
                    return Some(layout);
                }
                // Wrapper
                Layout::Padding { element, .. }
                | Layout::Explicit { element, .. }
                | Layout::Offset { element, .. }
                | Layout::Conditional { element, .. } => {
                    self.stack.push(element);
                    return Some(layout);
                }
            }
        }
        None
    }
}

impl<Context> Layout<Context> {
    fn iter(&self) -> LayoutNodeIterator<Context> {
        LayoutNodeIterator { stack: vec![self] }
    }

    pub fn visit_drawables(
        &self,
        ctx: &mut Context,
        visit: impl Fn(&Drawable<Context>, &mut Context),
    ) {
        for node in self.iter().filter_map(|l| {
            let Layout::Draw(d) = l else { return None };
            Some(d)
        }) {
            visit(node, ctx)
        }
    }

    pub fn drawables(&self) -> Vec<&Drawable<Context>> {
        self.iter()
            .filter_map(|l| {
                let Layout::Draw(d) = l else { return None };
                Some(d)
            })
            .collect::<Vec<&Drawable<Context>>>()
    }

    pub fn layout_draw(&mut self, available_area: Area, ctx: &mut Context) {
        self.layout(available_area);
        self.visit_drawables(ctx, |drawable, ctx| {
            (drawable.draw)(drawable.area, ctx);
        });
    }

    pub fn layout(&mut self, available_area: Area) {
        match self {
            Layout::Padding {
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
            Layout::Column { elements, spacing } => {
                layout_axis(elements, spacing, available_area, Orientation::Vertical)
            }
            Layout::Row { elements, spacing } => {
                layout_axis(elements, spacing, available_area, Orientation::Horizontal)
            }
            Layout::Stack(children) => {
                for child in children {
                    child.layout(available_area)
                }
            }
            Layout::Draw(drawable) => {
                drawable.area = Area {
                    x: available_area.x,
                    y: available_area.y,
                    width: available_area.width.max(0.),
                    height: available_area.height.max(0.),
                };
            }
            Layout::Explicit {
                options:
                    Size {
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
                    },
                element: child,
            } => {
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
            Layout::Offset {
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
            Layout::Conditional {
                condition: _,
                element,
            } => element.layout(available_area),
        }
    }
}

enum Orientation {
    Horizontal,
    Vertical,
}

fn layout_axis<Context>(
    elements: &mut [Layout<Context>],
    spacing: &f32,
    available_area: Area,
    orientation: Orientation,
) {
    let sizes: Vec<Option<f32>> = elements
        .iter()
        .map(|element| match element {
            Layout::Conditional {
                condition,
                element: _,
            } => {
                if *condition {
                    None
                } else {
                    Some(0.)
                }
            }
            Layout::Explicit {
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
