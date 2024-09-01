#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub(crate) leading: f32,
    pub(crate) trailing: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
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

#[derive(Debug, Clone, Copy)]
pub enum Align {
    TopLeading,
    TopCenter,
    TopTrailing,
    CenterTrailing,
    BottomTrailing,
    BottomCenter,
    BottomLeading,
    CenterLeading,
    CenterCenter,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

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
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.into();
        self.x_relative = false;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.into();
        self.y_relative = false;
        self
    }
    pub fn width_relative(mut self, ratio: f32) -> Self {
        self.width = ratio.into();
        self.x_relative = true;
        self
    }
    pub fn height_relative(mut self, ratio: f32) -> Self {
        self.height = ratio.into();
        self.y_relative = true;
        self
    }
    pub fn min_width(mut self, width: f32) -> Self {
        self.width_min = width.into();
        self
    }
    pub fn min_height(mut self, height: f32) -> Self {
        self.height_min = height.into();
        self
    }
    pub fn max_width(mut self, width: f32) -> Self {
        self.width_max = width.into();
        self
    }
    pub fn max_height(mut self, height: f32) -> Self {
        self.height_max = height.into();
        self
    }
    pub fn x_align(mut self, align: XAlign) -> Self {
        self.x_align = align;
        self
    }
    pub fn y_align(mut self, align: YAlign) -> Self {
        self.y_align = align;
        self
    }
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
