#[derive(Debug, Clone)]
pub struct Drawable<T> {
    pub area: Area,
    pub element: T,
}

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

#[derive(Debug, Clone, Copy, Default)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
