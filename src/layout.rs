use std::marker::PhantomData;

use crate::models::*;

pub enum Layout<D, C>
where
    D: Fn(Area, &mut C),
{
    Padding {
        amounts: Padding,
        element: Box<Layout<D, C>>,
    },
    Column {
        elements: Vec<Layout<D, C>>,
        spacing: f32,
    },
    Row {
        elements: Vec<Layout<D, C>>,
        spacing: f32,
    },
    Stack(Vec<Layout<D, C>>),
    Offset {
        offset_x: f32,
        offset_y: f32,
        element: Box<Layout<D, C>>,
    },
    Draw(Drawable<D, C>),
    Explicit {
        options: Size,
        element: Box<Layout<D, C>>,
    },
    Conditional {
        condition: bool,
        element: Box<Layout<D, C>>,
    },
}

pub struct Drawable<DrawFn, Context>
where
    DrawFn: Fn(Area, &mut Context),
{
    pub area: Area,
    pub(crate) draw: DrawFn,
    pub(crate) t: PhantomData<Context>,
}

impl<DrawFn, Context> Drawable<DrawFn, Context>
where
    DrawFn: Fn(Area, &mut Context),
{
    pub fn draw(&self, area: Area, ctx: &mut Context) {
        (self.draw)(area, ctx)
    }
}

pub struct DrawableIterator<'a, T, U>
where
    T: Fn(Area, &mut U),
{
    stack: Vec<&'a Layout<T, U>>,
}

impl<'a, T, U> Iterator for DrawableIterator<'a, T, U>
where
    T: Fn(Area, &mut U),
{
    type Item = &'a Layout<T, U>;

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

impl<DrawFn, Context> Layout<DrawFn, Context>
where
    DrawFn: Fn(Area, &mut Context),
{
    pub fn iter(&self) -> DrawableIterator<DrawFn, Context> {
        DrawableIterator { stack: vec![self] }
    }

    pub fn drawables(&self) -> Vec<&Drawable<DrawFn, Context>> {
        self.iter()
            .filter_map(|l| {
                let Layout::Draw(d) = l else { return None };
                Some(d)
            })
            .collect::<Vec<&Drawable<DrawFn, Context>>>()
    }

    pub fn layout_draw(&mut self, available_area: Area, ctx: &mut Context) {
        self.layout(available_area);
        for drawable in self.drawables() {
            drawable.draw(drawable.area, ctx);
        }
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
                if elements.len() == 1 {
                    for element in elements {
                        element.layout(Area {
                            x: available_area.x,
                            y: available_area.y,
                            width: available_area.width,
                            height: available_area.height,
                        })
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
                    child.layout(area);
                    current_y += child_height + *spacing;
                }
            }
            Layout::Row { elements, spacing } => {
                if elements.len() == 1 {
                    for element in elements {
                        element.layout(Area {
                            x: available_area.x,
                            y: available_area.y,
                            width: available_area.width,
                            height: available_area.height,
                        })
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
                    child.layout(area);
                    current_x += child_width + *spacing;
                }
            }
            Layout::Stack(children) => {
                for child in children {
                    child.layout(available_area)
                }
            }
            Layout::Draw(drawable) => {
                if available_area.width > 0. && available_area.height > 0. {
                    drawable.area = available_area;
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
            Layout::Conditional { condition, element } => {
                if *condition {
                    element.layout(available_area)
                }
            }
        }
    }
}
