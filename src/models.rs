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

/// An allocation of screen space as a rectangle
#[derive(Debug, Clone, Copy, Default)]
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Padding {
    pub(crate) leading: f32,
    pub(crate) trailing: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

/// A builder for specifying size constraints. Used with `modifiers::size`.
/// Create a `Size::new()` & add constraints such as `Size::new().width(10.)`
#[derive(Debug, Clone, Copy)]
pub(crate) struct Size {
    pub(crate) width: Option<f32>,
    pub(crate) width_min: Option<f32>,
    pub(crate) width_max: Option<f32>,
    pub(crate) height: Option<f32>,
    pub(crate) height_min: Option<f32>,
    pub(crate) height_max: Option<f32>,
    pub(crate) x_align: XAlign,
    pub(crate) y_align: YAlign,
    pub(crate) x_relative: bool,
    pub(crate) y_relative: bool,
}

impl Default for Size {
    fn default() -> Self {
        Self::new()
    }
}

impl Size {
    /// Creates a default size object to add constraints to
    pub(crate) fn new() -> Self {
        Size {
            width: None,
            width_min: None,
            width_max: None,
            height: None,
            height_min: None,
            height_max: None,
            x_align: XAlign::Center,
            y_align: YAlign::Center,
            x_relative: false,
            y_relative: false,
        }
    }
}
