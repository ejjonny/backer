use std::ops::RangeBounds;

use crate::{layout::Node, layout::NodeValue, models::*};

impl<U> Node<U> {
    /// Adds padding to the node along the leading edge
    pub fn pad_leading(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn pad_x(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn pad_trailing(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn pad_top(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn pad_y(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn pad_bottom(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn pad(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn offset_x(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn offset_y(self, amount: f32) -> Node<U> {
        Node {
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
    pub fn offset(self, offset_x: f32, offset_y: f32) -> Node<U> {
        Node {
            inner: NodeValue::Offset {
                offset_x,
                offset_y,
                element: Box::new(self.inner),
            },
        }
    }
    /// Specifies an explicit width for a node
    pub fn width(self, width: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.width = width.into();
            options.x_relative = false;
        })
    }
    /// Specifies an explicit height for a node
    pub fn height(self, height: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.height = height.into();
            options.y_relative = false;
        })
    }
    /// Specifies bounds on a node's height
    pub fn height_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        self.wrap_or_update_explicit(|options| {
            match range.start_bound() {
                std::ops::Bound::Included(bound) => options.height_min = Some(*bound),
                std::ops::Bound::Excluded(bound) => options.height_min = Some(*bound),
                std::ops::Bound::Unbounded => (),
            }
            match range.end_bound() {
                std::ops::Bound::Included(bound) => options.height_max = Some(*bound),
                std::ops::Bound::Excluded(bound) => options.height_max = Some(*bound),
                std::ops::Bound::Unbounded => (),
            }
        })
    }
    /// Specifies bounds on a node's width
    pub fn width_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        self.wrap_or_update_explicit(|options| {
            match range.start_bound() {
                std::ops::Bound::Included(bound) => options.width_min = Some(*bound),
                std::ops::Bound::Excluded(bound) => options.width_min = Some(*bound),
                std::ops::Bound::Unbounded => (),
            }
            match range.end_bound() {
                std::ops::Bound::Included(bound) => options.width_max = Some(*bound),
                std::ops::Bound::Excluded(bound) => options.width_max = Some(*bound),
                std::ops::Bound::Unbounded => (),
            }
        })
    }
    /// Specifies an alignment along the x axis.
    ///
    /// This will only have an effect if the node is constrained to be smaller than the area that is available,
    /// otherwise, there's no wiggle room.
    pub fn x_align(mut self, align: XAlign) -> Self {
        match self.inner {
            NodeValue::Column {
                align: ref mut col_align,
                ..
            } => *col_align = align,
            _ => {
                return self.wrap_or_update_explicit(|options| {
                    options.x_align = Some(align);
                })
            }
        }
        self
    }
    /// Specifies an alignment along the y axis.
    ///
    /// This will only have an effect if the node is constrained to be smaller than the area that is available,
    /// otherwise, there's no wiggle room.
    pub fn y_align(mut self, align: YAlign) -> Self {
        match self.inner {
            NodeValue::Row {
                align: ref mut row_align,
                ..
            } => *row_align = align,
            _ => {
                return self.wrap_or_update_explicit(|options| {
                    options.y_align = Some(align);
                })
            }
        }
        self
    }
    /// Specifies an alignment along both the x & y axis.
    ///
    /// This will only have an effect if the node is constrained along the axis to be smaller than the area that is available,
    /// otherwise, there's no wiggle room.
    pub fn align(self, align: Align) -> Self {
        let (x_align, y_align) = match align {
            Align::TopLeading => (XAlign::Leading, YAlign::Top),
            Align::TopCenter => (XAlign::Center, YAlign::Top),
            Align::TopTrailing => (XAlign::Trailing, YAlign::Top),
            Align::CenterTrailing => (XAlign::Trailing, YAlign::Center),
            Align::BottomTrailing => (XAlign::Trailing, YAlign::Bottom),
            Align::BottomCenter => (XAlign::Center, YAlign::Bottom),
            Align::BottomLeading => (XAlign::Leading, YAlign::Bottom),
            Align::CenterLeading => (XAlign::Leading, YAlign::Center),
            Align::CenterCenter => (XAlign::Center, YAlign::Center),
        };
        self.x_align(x_align).y_align(y_align)
    }

    /// Constrains the node's height to `ratio` of width
    pub fn aspect(self, ratio: f32) -> Self {
        self.wrap_or_update_explicit(|options| options.aspect = Some(ratio))
    }

    fn wrap_or_update_explicit(mut self, update: impl Fn(&mut Size)) -> Self {
        match self.inner {
            NodeValue::Explicit {
                ref mut options,
                element: _,
            } => {
                update(options);
            }
            _ => {
                let mut options = Size::new();
                update(&mut options);
                return Node {
                    inner: NodeValue::Explicit {
                        options,
                        element: Box::new(self.inner),
                    },
                };
            }
        }
        self
    }
}
