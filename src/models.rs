use std::rc::Rc;

/// Alignment along the X axis
#[derive(Debug, Clone, Copy)]
pub enum XAlign {
    /// Aligns to the left in LTR layout
    Leading,
    /// Aligns to the horizontal center
    Center,
    /// Aligns to the right in LTR layout
    Trailing,
}

impl From<YAlign> for (Option<XAlign>, Option<YAlign>) {
    fn from(value: YAlign) -> Self {
        (None, Some(value))
    }
}

/// Alignment along the Y axis
#[derive(Debug, Clone, Copy)]
pub enum YAlign {
    /// Aligns to the top
    Top,
    /// Aligns to the vertical center
    Center,
    /// Aligns to the bottom
    Bottom,
}

impl From<XAlign> for (Option<XAlign>, Option<YAlign>) {
    fn from(value: XAlign) -> Self {
        (Some(value), None)
    }
}

/// An alignment along both the X and Y axis
#[derive(Debug, Clone, Copy)]
pub enum Align {
    /// Aligns to the top left in LTR layout
    TopLeading,
    /// Aligns to the top center
    TopCenter,
    /// Aligns to the top right in LTR layout
    TopTrailing,
    /// Aligns to the middle right in LTR layout
    CenterTrailing,
    /// Aligns to the bottom right in LTR layout
    BottomTrailing,
    /// Aligns to the bottom middle
    BottomCenter,
    /// Aligns to the bottom left in LTR layout
    BottomLeading,
    /// Aligns to the middle left in LTR layout
    CenterLeading,
    /// Aligns to the center in LTR layout - the default alignment
    CenterCenter,
}

impl From<Align> for (Option<XAlign>, Option<YAlign>) {
    fn from(value: Align) -> Self {
        let (x_align, y_align) = match value {
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
        (Some(x_align), Some(y_align))
    }
}

/// An allocation of screen space as a rectangle
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Area {
    /// Origin - usually the left-most X
    pub x: f32,
    /// Origin - usually the upper-most Y
    pub y: f32,
    /// Available width, starting at `x`
    pub width: f32,
    /// Available height, starting at `y`
    pub height: f32,
}

impl Area {
    /// Creates a new [`Area`].
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    #[allow(unused)]
    pub(crate) fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Padding {
    pub(crate) leading: f32,
    pub(crate) trailing: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

type DimensionFn<U> = Option<Rc<dyn Fn(f32, &mut U) -> f32>>;

pub(crate) struct Size<U> {
    pub(crate) width_min: Option<f32>,
    pub(crate) width_max: Option<f32>,
    pub(crate) height_min: Option<f32>,
    pub(crate) height_max: Option<f32>,
    pub(crate) x_align: Option<XAlign>,
    pub(crate) y_align: Option<YAlign>,
    pub(crate) aspect: Option<f32>,
    pub(crate) dynamic_height: DimensionFn<U>,
    pub(crate) dynamic_width: DimensionFn<U>,
}

impl<U> Clone for Size<U> {
    fn clone(&self) -> Self {
        Self {
            width_min: self.width_min,
            width_max: self.width_max,
            height_min: self.height_min,
            height_max: self.height_max,
            x_align: self.x_align,
            y_align: self.y_align,
            aspect: self.aspect,
            dynamic_height: self.dynamic_height.clone(),
            dynamic_width: self.dynamic_width.clone(),
        }
    }
}

impl<U> std::fmt::Debug for Size<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size")
    }
}

impl<U> Default for Size<U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<U> Size<U> {
    /// Creates a default size object to add constraints to
    pub(crate) fn new() -> Self {
        Size {
            width_min: None,
            width_max: None,
            height_min: None,
            height_max: None,
            x_align: None,
            y_align: None,
            aspect: None,
            dynamic_height: None,
            dynamic_width: None,
        }
    }
}
