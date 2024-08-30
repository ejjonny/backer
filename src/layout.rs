use crate::models::*;

#[derive(Debug, Clone)]
pub enum Layout<T> {
    Padding {
        amounts: Padding,
        child: Box<Layout<T>>,
    },
    Column {
        elements: Vec<Layout<T>>,
        spacing: f32,
    },
    Row {
        elements: Vec<Layout<T>>,
        spacing: f32,
    },
    Stack(Vec<Layout<T>>),
    Draw(Drawable<T>),
    Explicit {
        w: Option<f32>,
        h: Option<f32>,
        x_align: XAlign,
        y_align: YAlign,
        ratio: bool,
        child: Box<Layout<T>>,
    },
}

impl<T> Layout<T> {
    pub fn drawables(&self) -> Vec<&Drawable<T>> {
        let mut drawables = Vec::new();
        let mut stack = vec![self];

        while let Some(layout) = stack.pop() {
            match layout {
                Layout::Draw(drawable) => drawables.push(drawable),
                Layout::Padding { child, .. } => stack.push(child),
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
                Layout::Explicit { child, .. } => stack.push(child),
            }
        }
        drawables
    }

    pub fn layout(&mut self, available_area: Area) {
        match self {
            Layout::Padding { amounts, child } => {
                let inner_area = Area {
                    x: available_area.x + amounts.leading,
                    y: available_area.y + amounts.top,
                    width: available_area.width - amounts.trailing - amounts.leading,
                    height: available_area.height - amounts.bottom - amounts.top,
                };
                child.layout(inner_area);
            }
            Layout::Column { elements, spacing } => {
                let total_spacing = *spacing * (elements.len() - 1) as f32;
                let available_height = available_area.height - total_spacing;
                let child_height = available_height / elements.len() as f32;

                let mut current_y = available_area.y;
                for child in elements.iter_mut() {
                    let child_area = Area {
                        x: available_area.x,
                        y: current_y,
                        width: available_area.width,
                        height: child_height,
                    };
                    child.layout(child_area);
                    current_y += child_height + *spacing;
                }
            }
            Layout::Row { elements, spacing } => {
                let total_spacing = *spacing * (elements.len() - 1) as f32;
                let available_width = available_area.width - total_spacing;
                let child_width = available_width / elements.len() as f32;

                let mut current_x = available_area.x;
                for child in elements.iter_mut() {
                    let child_area = Area {
                        x: current_x,
                        y: available_area.y,
                        width: child_width,
                        height: available_area.height,
                    };
                    child.layout(child_area);
                    current_x += child_width + *spacing;
                }
            }
            Layout::Stack(children) => {
                for child in children {
                    child.layout(available_area)
                }
            }
            Layout::Draw(item) => {
                if available_area.width > 0. && available_area.height > 0. {
                    item.area = available_area
                }
            }
            Layout::Explicit {
                w: width,
                h: height,
                x_align,
                y_align,
                ratio,
                child,
            } => {
                let explicit_width = if *ratio {
                    available_area.width * width.unwrap_or(1.0)
                } else {
                    width.unwrap_or(available_area.width)
                };
                let explicit_height = if *ratio {
                    available_area.height * height.unwrap_or(1.0)
                } else {
                    height.unwrap_or(available_area.height)
                };
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
                    width: width
                        .unwrap_or(available_area.width)
                        .min(available_area.width),
                    height: height
                        .unwrap_or(available_area.height)
                        .min(available_area.height),
                });
            }
        }
    }
}
