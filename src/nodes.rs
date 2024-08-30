use crate::{layout::Layout, models::*};

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
        area: Area::default(),
        element: drawable,
    })
}
