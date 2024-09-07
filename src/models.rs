#[derive(Debug, Clone, Copy)]
pub(crate) struct Padding {
    pub(crate) leading: f32,
    pub(crate) trailing: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

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

/// A builder for specifying size constraints. Used with `modifiers::size`.
/// Create a `Size::new()` & add constraints such as `Size::new().width(10.)`
#[derive(Debug, Clone, Copy)]
pub struct Size {
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
    pub fn new() -> Self {
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
    /// Specifies an explicit width for a node
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.into();
        self.x_relative = false;
        self
    }
    /// Specifies an explicit height for a node
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.into();
        self.y_relative = false;
        self
    }
    /// Specifies an explicit width for a node as a fraction of the available width
    pub fn width_relative(mut self, ratio: f32) -> Self {
        self.width = ratio.into();
        self.x_relative = true;
        self
    }
    /// Specifies an explicit height for a node as a fraction of the available height
    pub fn height_relative(mut self, ratio: f32) -> Self {
        self.height = ratio.into();
        self.y_relative = true;
        self
    }
    /// Specifies a lower bound on a node's width
    pub fn min_width(mut self, width: f32) -> Self {
        self.width_min = width.into();
        self
    }
    /// Specifies a lower bound on a node's height
    pub fn min_height(mut self, height: f32) -> Self {
        self.height_min = height.into();
        self
    }
    /// Specifies an upper bound on a node's width
    pub fn max_width(mut self, width: f32) -> Self {
        self.width_max = width.into();
        self
    }
    /// Specifies an upper bound on a node's height
    pub fn max_height(mut self, height: f32) -> Self {
        self.height_max = height.into();
        self
    }
    /// Specifies an alignment along the x axis.
    /// This will only have an effect if the node is constrained to be smaller than the area that is available
    /// Otherwise, there's no wiggle room!
    pub fn x_align(mut self, align: XAlign) -> Self {
        self.x_align = align;
        self
    }
    /// Specifies an alignment along the y axis.
    /// This will only have an effect if the node is constrained to be smaller than the area that is available
    /// Otherwise, there's no wiggle room!
    pub fn y_align(mut self, align: YAlign) -> Self {
        self.y_align = align;
        self
    }
    /// Specifies an alignment along both the x & y axis.
    /// This will only have an effect if the node is constrained along the axis to be smaller than the area that is available
    /// Otherwise, there's no wiggle room!
    pub fn align(mut self, align: Align) -> Self {
        match align {
            Align::TopLeading => {
                self.y_align = YAlign::Top;
                self.x_align = XAlign::Leading;
            }
            Align::TopCenter => {
                self.y_align = YAlign::Top;
                self.x_align = XAlign::Center;
            }
            Align::TopTrailing => {
                self.y_align = YAlign::Top;
                self.x_align = XAlign::Trailing;
            }
            Align::CenterTrailing => {
                self.y_align = YAlign::Center;
                self.x_align = XAlign::Trailing;
            }
            Align::BottomTrailing => {
                self.y_align = YAlign::Bottom;
                self.x_align = XAlign::Trailing;
            }
            Align::BottomCenter => {
                self.y_align = YAlign::Bottom;
                self.x_align = XAlign::Center;
            }
            Align::BottomLeading => {
                self.y_align = YAlign::Bottom;
                self.x_align = XAlign::Leading;
            }
            Align::CenterLeading => {
                self.y_align = YAlign::Center;
                self.x_align = XAlign::Leading;
            }
            Align::CenterCenter => {
                self.y_align = YAlign::Center;
                self.x_align = XAlign::Center;
            }
        }
        self
    }
}
