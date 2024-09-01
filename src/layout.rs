use crate::models::*;

pub enum Layout<'a, T> {
    Padding {
        amounts: Padding,
        element: Box<Layout<'a, T>>,
    },
    Column {
        elements: Vec<Layout<'a, T>>,
        spacing: f32,
    },
    Row {
        elements: Vec<Layout<'a, T>>,
        spacing: f32,
    },
    Stack(Vec<Layout<'a, T>>),
    Offset {
        offset_x: f32,
        offset_y: f32,
        element: Box<Layout<'a, T>>,
    },
    Draw(Drawable<'a, T>),
    Explicit {
        options: Size,
        element: Box<Layout<'a, T>>,
    },
    Conditional {
        condition: bool,
        element: Box<Layout<'a, T>>,
    },
}

type DrawableFn<'a, T> = Box<dyn FnMut(Area, &mut T) + 'a>;

pub struct Drawable<'a, T> {
    pub(crate) area: Area,
    pub(crate) draw: DrawableFn<'a, T>,
}

impl<'a, T> Layout<'a, T> {
    pub fn drawables(&self) -> Vec<&Drawable<'a, T>> {
        let mut drawables = Vec::new();
        let mut stack = vec![self];

        while let Some(layout) = stack.pop() {
            match layout {
                Layout::Draw(drawable) => drawables.push(drawable),
                Layout::Padding { element: child, .. } => stack.push(child),
                Layout::Column {
                    elements,
                    spacing: _,
                }
                | Layout::Row {
                    elements,
                    spacing: _,
                }
                | Layout::Stack(elements) => {
                    stack.extend(elements.iter().rev());
                }
                Layout::Explicit { element, .. } => stack.push(element),
                Layout::Offset {
                    offset_x: _,
                    offset_y: _,
                    element,
                } => stack.push(element),
                Layout::Conditional { condition, element } => {
                    if *condition {
                        stack.push(element)
                    }
                }
            }
        }
        drawables
    }

    pub fn layout(&mut self, available_area: Area, ctx: &mut T) {
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
                child.layout(inner_area, ctx);
            }
            Layout::Column { elements, spacing } => {
                if elements.len() == 1 {
                    for element in elements {
                        element.layout(
                            Area {
                                x: available_area.x,
                                y: available_area.y,
                                width: available_area.width,
                                height: available_area.height,
                            },
                            ctx,
                        )
                    }
                    return;
                }
                let total_spacing = *spacing * (elements.len() as i32 - 1).max(0) as f32;
                let available_height = available_area.height - total_spacing;
                let heights: Vec<Option<f32>> = elements
                    .iter()
                    .map(|e| {
                        let Layout::Explicit {
                            options,
                            element: _,
                        } = e
                        else {
                            return None;
                        };
                        if options.y_relative {
                            return None;
                        }
                        options.height?.into()
                    })
                    .collect();
                let explicit_consumed = heights.iter().filter_map(|&h| h).fold(0., |c, n| c + n);
                let remaining = available_height - explicit_consumed;
                let unconstrained_heights = heights
                    .iter()
                    .filter(|&h| h.is_none())
                    .collect::<Vec<&Option<f32>>>()
                    .len();
                let default_height = remaining / unconstrained_heights as f32;

                let mut current_y = available_area.y;
                for (i, child) in elements.iter_mut().enumerate() {
                    let child_height = heights
                        .get(i)
                        .unwrap_or(&Some(default_height))
                        .unwrap_or(default_height);
                    let area = Area {
                        x: available_area.x,
                        y: current_y,
                        width: available_area.width,
                        height: child_height,
                    };
                    child.layout(area, ctx);
                    current_y += child_height + *spacing;
                }
            }
            Layout::Row { elements, spacing } => {
                if elements.len() == 1 {
                    for element in elements {
                        element.layout(
                            Area {
                                x: available_area.x,
                                y: available_area.y,
                                width: available_area.width,
                                height: available_area.height,
                            },
                            ctx,
                        )
                    }
                    return;
                }
                let total_spacing = *spacing * (elements.len() as i32 - 1).max(0) as f32;
                let available_width = available_area.width - total_spacing;
                let widths: Vec<Option<f32>> = elements
                    .iter()
                    .map(|e| {
                        let Layout::Explicit {
                            options,
                            element: _,
                        } = e
                        else {
                            return None;
                        };
                        if options.x_relative {
                            return None;
                        }
                        options.width?.into()
                    })
                    .collect();
                let explicit_consumed = widths.iter().filter_map(|&h| h).fold(0., |c, n| c + n);
                let remaining = available_width - explicit_consumed;
                let unconstrained_widths = widths
                    .iter()
                    .filter(|&h| h.is_none())
                    .collect::<Vec<&Option<f32>>>()
                    .len();
                let default_width = remaining / unconstrained_widths as f32;

                let mut current_x = available_area.x;
                for (i, child) in elements.iter_mut().enumerate() {
                    let child_width = widths
                        .get(i)
                        .unwrap_or(&Some(default_width))
                        .unwrap_or(default_width);
                    let area = Area {
                        x: current_x,
                        y: available_area.y,
                        width: child_width,
                        height: available_area.height,
                    };
                    child.layout(area, ctx);
                    current_x += child_width + *spacing;
                }
            }
            Layout::Stack(children) => {
                for child in children {
                    child.layout(available_area, ctx)
                }
            }
            Layout::Draw(drawable) => {
                if available_area.width > 0. && available_area.height > 0. {
                    drawable.area = available_area;
                    (drawable.draw)(available_area, ctx);
                }
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
                child.layout(
                    Area {
                        x: x.max(available_area.x),
                        y: y.max(available_area.y),
                        width: explicit_width,
                        height: explicit_height,
                    },
                    ctx,
                );
            }
            Layout::Offset {
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
                    ctx,
                );
            }
            Layout::Conditional { condition, element } => {
                if *condition {
                    element.layout(available_area, ctx)
                }
            }
        }
    }
}
