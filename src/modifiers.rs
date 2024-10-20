use crate::{layout::NodeValue, models::*, NodeWith};
use std::{ops::RangeBounds, rc::Rc};

impl<State> NodeWith<State, ()> {
    /// Constrains the node's height as a function of available width.
    ///
    /// Generally you should prefer size constraints, aspect ratio constraints or area readers over dynamic height.
    ///
    /// **This is primarily for UI elements such as text** where node height must depend on available width & scaling is
    /// not a simple option.
    pub fn dynamic_height(self, f: impl Fn(f32, &mut State) -> f32 + 'static) -> Self {
        self.wrap_or_update_explicit(Size {
            dynamic_height: Some(Rc::new(move |h, a, _| f(h, a))),
            ..Default::default()
        })
    }
    /// Constrains the node's width as a function of available height.
    ///
    /// Generally you should prefer size constraints, aspect ratio constraints or area readers over dynamic height.
    ///
    /// **This is primarily for UI elements such as text** where node width must depend on available height & scaling is
    /// not a simple option.
    pub fn dynamic_width(self, f: impl Fn(f32, &mut State) -> f32 + 'static) -> Self {
        self.wrap_or_update_explicit(Size {
            dynamic_width: Some(Rc::new(move |h, a, _| f(h, a))),
            ..Default::default()
        })
    }
}

impl<State, Ctx> NodeWith<State, Ctx> {
    /// Constrains the node's height as a function of available width.
    ///
    /// Generally you should prefer size constraints, aspect ratio constraints or area readers over dynamic height.
    ///
    /// **This is primarily for UI elements such as text** where node height must depend on available width & scaling is
    /// not a simple option.
    pub fn dynamic_height_with(
        self,
        f: impl Fn(f32, &mut State, &mut Ctx) -> f32 + 'static,
    ) -> Self {
        self.wrap_or_update_explicit(Size {
            dynamic_height: Some(Rc::new(f)),
            ..Default::default()
        })
    }
    /// Constrains the node's width as a function of available height.
    ///
    /// Generally you should prefer size constraints, aspect ratio constraints or area readers over dynamic height.
    ///
    /// **This is primarily for UI elements such as text** where node width must depend on available height & scaling is
    /// not a simple option.
    pub fn dynamic_width_with(
        self,
        f: impl Fn(f32, &mut State, &mut Ctx) -> f32 + 'static,
    ) -> Self {
        self.wrap_or_update_explicit(Size {
            dynamic_width: Some(Rc::new(f)),
            ..Default::default()
        })
    }
    /// Adds padding to the node along the leading edge
    pub fn pad_leading(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: 0.,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds horizontal padding to the node (leading & trailing)
    pub fn pad_x(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: amount,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node along the trailing edge
    pub fn pad_trailing(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: amount,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node along the top edge
    pub fn pad_top(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: amount,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }

    /// Adds vertical padding to the node (top & bottom)
    pub fn pad_y(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: amount,
                    bottom: amount,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node along the bottom edge
    pub fn pad_bottom(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: 0.,
                    bottom: amount,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node on all sides
    pub fn pad(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: amount,
                    top: amount,
                    bottom: amount,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Offsets the node along the x axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_x(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Offset {
                offset_x: amount,
                offset_y: 0.,
                element: Box::new(self.inner),
            },
        }
    }
    /// Offsets the node along the y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_y(self, amount: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Offset {
                offset_x: 0.,
                offset_y: amount,
                element: Box::new(self.inner),
            },
        }
    }
    /// Offsets the node along the x & y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset(self, offset_x: f32, offset_y: f32) -> NodeWith<State, Ctx> {
        NodeWith {
            inner: NodeValue::Offset {
                offset_x,
                offset_y,
                element: Box::new(self.inner),
            },
        }
    }
    /// Specifies an explicit width for a node
    pub fn width(self, width: f32) -> Self {
        self.wrap_or_update_explicit(Size {
            width_min: Some(width),
            width_max: Some(width),
            ..Default::default()
        })
    }
    /// Specifies an explicit height for a node
    pub fn height(self, height: f32) -> Self {
        self.wrap_or_update_explicit(Size {
            height_min: Some(height),
            height_max: Some(height),
            ..Default::default()
        })
    }
    /// Specifies bounds on a node's height
    pub fn height_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        self.wrap_or_update_explicit(Size {
            height_min: match range.start_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            height_max: match range.end_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            ..Default::default()
        })
    }
    /// Specifies bounds on a node's width
    pub fn width_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        self.wrap_or_update_explicit(Size {
            width_min: match range.start_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            width_max: match range.end_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            ..Default::default()
        })
    }
    /// Specifies an alignment along the x and/or y axis.
    ///
    /// This will only have an effect if the node is constrained along the axis to be smaller than the area that is available,
    /// otherwise, there's no wiggle room.
    pub fn align(self, align: Align) -> Self {
        let (x, y) = align.axis_aligns();
        match (x, y) {
            (None, None) => self,
            (None, Some(y)) => self.y_align(y),
            (Some(x), None) => self.x_align(x),
            (Some(x), Some(y)) => self.x_align(x).y_align(y),
        }
    }
    fn x_align(mut self, align: XAlign) -> Self {
        match self.inner {
            NodeValue::Column {
                off_axis_align: ref mut col_align,
                ..
            } => *col_align = Some(align),
            NodeValue::Row {
                align: ref mut row_align,
                ..
            } => *row_align = Some(align),
            _ => {
                return self.wrap_or_update_explicit(Size {
                    x_align: Some(align),
                    ..Default::default()
                });
            }
        }
        self
    }
    fn y_align(mut self, align: YAlign) -> Self {
        match self.inner {
            NodeValue::Row {
                off_axis_align: ref mut row_align,
                ..
            } => *row_align = Some(align),
            NodeValue::Column {
                align: ref mut col_align,
                ..
            } => *col_align = Some(align),
            _ => {
                return self.wrap_or_update_explicit(Size {
                    y_align: Some(align),
                    ..Default::default()
                });
            }
        }
        self
    }
    /// Constrains the node's height to `ratio` of width
    pub fn aspect(self, ratio: f32) -> Self {
        self.wrap_or_update_explicit(Size {
            aspect: Some(ratio),
            ..Default::default()
        })
    }
    /// Attaches `node` over this node as an overlay
    ///
    /// The area available to the attached node is the size of the node it's attached to.
    /// Useful for adding an unconstrained node as an ornament, background, or overlay to a constrained node.
    pub fn attach_over(self, node: Self) -> Self {
        NodeWith {
            inner: NodeValue::Coupled {
                over: true,
                element: Box::new(self.inner),
                coupled: Box::new(node.inner),
            },
        }
    }
    /// Attaches `node` under this node as a background
    ///
    /// The area available to the attached node is the size of the node it's attached to.
    /// Useful for adding an unconstrained node as an ornament, background, or overlay to a constrained node.
    pub fn attach_under(self, node: Self) -> Self {
        NodeWith {
            inner: NodeValue::Coupled {
                over: false,
                element: Box::new(self.inner),
                coupled: Box::new(node.inner),
            },
        }
    }
    fn wrap_or_update_explicit(mut self, size: Size<State, Ctx>) -> Self {
        match self.inner {
            NodeValue::Explicit {
                ref mut options,
                element: _,
            } => {
                let width_update = size.width_min.or(size.width_max).is_some();
                let height_update = size.height_min.or(size.height_max).is_some();
                *options = Size {
                    width_min: if width_update {
                        size.width_min
                    } else {
                        options.width_min
                    },
                    width_max: if width_update {
                        size.width_max
                    } else {
                        options.width_max
                    },
                    height_min: if height_update {
                        size.height_min
                    } else {
                        options.height_min
                    },
                    height_max: if height_update {
                        size.height_min
                    } else {
                        options.height_min
                    },
                    x_align: size.x_align.or(options.x_align),
                    y_align: size.y_align.or(options.y_align),
                    aspect: size.aspect.or(options.aspect),
                    dynamic_height: size.dynamic_height.or(options.dynamic_height.clone()),
                    dynamic_width: size.dynamic_width.or(options.dynamic_width.clone()),
                };
            }
            _ => {
                return NodeWith {
                    inner: NodeValue::Explicit {
                        options: size,
                        element: Box::new(self.inner),
                    },
                };
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::nodes::*;

    #[test]
    fn test_explicit_wrap_valid() {
        let c = space::<(), ()>()
            .width(10.)
            .width_range(5.0..)
            .inner
            .constraints(Area::zero(), &mut (), &mut ());
        assert!(c.width.get_upper().is_none());
        assert_eq!(c.width.get_lower().unwrap(), 5.);
    }
}
