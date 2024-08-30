use crate::{layout::Layout, models::*};

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
