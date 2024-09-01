use crate::{layout::Layout, models::*};

impl<'a, T> Layout<'a, T> {
    pub fn pad_leading(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: amount,
                trailing: 0.,
                top: 0.,
                bottom: 0.,
            },
            element: Box::new(self),
        }
    }

    pub fn pad_x(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: amount,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            element: Box::new(self),
        }
    }

    pub fn pad_trailing(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            element: Box::new(self),
        }
    }

    pub fn pad_top(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: 0.,
            },
            element: Box::new(self),
        }
    }

    pub fn pad_y(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: amount,
            },
            element: Box::new(self),
        }
    }

    pub fn pad_bottom(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: 0.,
                trailing: 0.,
                top: 0.,
                bottom: amount,
            },
            element: Box::new(self),
        }
    }

    pub fn pad(self, amount: f32) -> Layout<'a, T> {
        Layout::Padding {
            amounts: Padding {
                leading: amount,
                trailing: amount,
                top: amount,
                bottom: amount,
            },
            element: Box::new(self),
        }
    }

    pub fn size(self, options: Size) -> Layout<'a, T> {
        Layout::Explicit {
            options,
            element: Box::new(self),
        }
    }

    pub fn offset_x(self, amount: f32) -> Layout<'a, T> {
        Layout::Offset {
            offset_x: amount,
            offset_y: 0.,
            element: Box::new(self),
        }
    }

    pub fn offset_y(self, amount: f32) -> Layout<'a, T> {
        Layout::Offset {
            offset_x: 0.,
            offset_y: amount,
            element: Box::new(self),
        }
    }

    pub fn offset(self, offset_x: f32, offset_y: f32) -> Layout<'a, T> {
        Layout::Offset {
            offset_x,
            offset_y,
            element: Box::new(self),
        }
    }
}
