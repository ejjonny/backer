#[derive(Debug, Clone)]
pub enum Layout<T> {
    Padding {
        amounts: Padding,
        child: Box<Layout<T>>,
    },
    Column(Vec<Layout<T>>),
    Row(Vec<Layout<T>>),
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

#[derive(Debug, Clone)]
pub struct Drawable<T> {
    pub area: Area,
    pub element: T,
}

impl<T> Layout<T> {
    pub fn pad_leading(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: amount,
                trailing: 0.,
                top: 0.,
                bottom: 0.,
            },
            child: Box::new(self),
        }
    }

    pub fn pad_x(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: amount,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            child: Box::new(self),
        }
    }

    pub fn pad_trailing(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            child: Box::new(self),
        }
    }

    pub fn pad_top(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: 0.,
            },
            child: Box::new(self),
        }
    }

    pub fn pad_y(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: amount,
            },
            child: Box::new(self),
        }
    }

    pub fn pad_bottom(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: 0.,
                top: 0.,
                bottom: amount,
            },
            child: Box::new(self),
        }
    }

    pub fn pad(self, amount: f32) -> Layout<T> {
        Layout::Padding {
            amounts: Padding {
                leading: amount,
                trailing: amount,
                top: amount,
                bottom: amount,
            },
            child: Box::new(self),
        }
    }

    pub fn height(self, h: f32, align: YAlign) -> Layout<T> {
        Layout::Explicit {
            w: None,
            h: Some(h),
            x_align: XAlign::Center,
            y_align: align,
            ratio: false,
            child: Box::new(self),
        }
    }

    pub fn width(self, w: f32, align: XAlign) -> Layout<T> {
        Layout::Explicit {
            w: Some(w),
            h: None,
            x_align: align,
            y_align: YAlign::Center,
            ratio: false,
            child: Box::new(self),
        }
    }

    pub fn size(self, w: f32, h: f32, x_align: XAlign, y_align: YAlign) -> Layout<T> {
        Layout::Explicit {
            w: Some(w),
            h: Some(h),
            x_align,
            y_align,
            ratio: false,
            child: Box::new(self),
        }
    }

    pub fn height_ratio(self, h: f32, align: YAlign) -> Layout<T> {
        Layout::Explicit {
            w: None,
            h: Some(h),
            x_align: XAlign::Center,
            y_align: align,
            ratio: true,
            child: Box::new(self),
        }
    }

    pub fn width_ratio(self, w: f32, align: XAlign) -> Layout<T> {
        Layout::Explicit {
            w: Some(w),
            h: None,
            x_align: align,
            y_align: YAlign::Center,
            ratio: true,
            child: Box::new(self),
        }
    }

    pub fn size_ratio(self, w: f32, h: f32, x_align: XAlign, y_align: YAlign) -> Layout<T> {
        Layout::Explicit {
            w: Some(w),
            h: Some(h),
            x_align,
            y_align,
            ratio: true,
            child: Box::new(self),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    leading: f32,
    trailing: f32,
    top: f32,
    bottom: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum XAlign {
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Clone, Copy)]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}

impl<T> Layout<T> {
    pub fn drawables(&self) -> Vec<&Drawable<T>> {
        let mut drawables = Vec::new();
        let mut stack = vec![self];

        while let Some(layout) = stack.pop() {
            match layout {
                Layout::Draw(drawable) => drawables.push(drawable),
                Layout::Padding { child, .. } => stack.push(child),
                Layout::Column(children) | Layout::Row(children) | Layout::Stack(children) => {
                    stack.extend(children.iter().rev());
                }
                Layout::Explicit { child, .. } => stack.push(child),
            }
        }
        drawables
    }
    pub fn layout(&mut self, with: Area) {
        match self {
            Layout::Padding { amounts, child } => {
                let inner_area = Area {
                    x: with.x + amounts.leading,
                    y: with.y + amounts.top,
                    width: with.width - amounts.trailing - amounts.leading,
                    height: with.height - amounts.bottom - amounts.top,
                };
                child.layout(inner_area);
            }
            Layout::Column(children) => {
                let child_height = with.height / children.len() as f32;
                for (i, child) in children.iter_mut().enumerate() {
                    let child_y = with.y + i as f32 * child_height;
                    let child_area = Area {
                        x: with.x,
                        y: child_y,
                        width: with.width,
                        height: child_height,
                    };
                    child.layout(child_area);
                }
            }
            Layout::Row(children) => {
                let child_width = with.width / children.len() as f32;
                for (i, child) in children.iter_mut().enumerate() {
                    let child_x = with.x + i as f32 * child_width;
                    let child_area = Area {
                        x: child_x,
                        y: with.y,
                        width: child_width,
                        height: with.height,
                    };
                    child.layout(child_area);
                }
            }
            Layout::Stack(children) => {
                for child in children {
                    child.layout(with)
                }
            }
            Layout::Draw(item) => {
                if with.width > 0. && with.height > 0. {
                    item.area = with
                }
            }
            Layout::Explicit {
                w,
                h,
                x_align,
                y_align,
                ratio,
                child,
            } => {
                let explicit_width = if *ratio {
                    with.width * w.unwrap_or(with.width)
                } else {
                    w.unwrap_or(with.width)
                };
                let explicit_height = if *ratio {
                    with.height * h.unwrap_or(with.height)
                } else {
                    h.unwrap_or(with.height)
                };
                let x = match x_align {
                    XAlign::Leading => with.x,
                    XAlign::Trailing => with.x + (with.width - explicit_width),
                    XAlign::Center => with.x + (with.width * 0.5) - (explicit_width * 0.5),
                };
                let y = match y_align {
                    YAlign::Top => with.y,
                    YAlign::Bottom => with.y + (with.height - explicit_height),
                    YAlign::Center => with.y + (with.height * 0.5) - (explicit_height * 0.5),
                };
                child.layout(Area {
                    x: x.max(with.x),
                    y: y.max(with.y),
                    width: w.unwrap_or(with.width).min(with.width),
                    height: h.unwrap_or(with.height).min(with.height),
                });
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn column<T>(children: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column(children)
}

pub fn row<T>(children: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row(children)
}

pub fn stack<T>(children: Vec<Layout<T>>) -> Layout<T> {
    Layout::Stack(children)
}

pub fn draw<T>(drawable: T) -> Layout<T> {
    Layout::Draw(Drawable {
        area: Area {
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
        },
        element: drawable,
    })
}
