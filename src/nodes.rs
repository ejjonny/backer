use crate::{layout::Layout, models::*};

pub fn column<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column {
        elements,
        spacing: 0.,
    }
}

pub fn column_spaced<T>(spacing: f32, elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column { elements, spacing }
}

pub fn row<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row {
        elements,
        spacing: 0.,
    }
}

pub fn row_spaced<T>(spacing: f32, elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row { elements, spacing }
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
