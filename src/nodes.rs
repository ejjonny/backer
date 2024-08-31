use crate::{layout::Layout, models::*};

pub fn column<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn column_spaced<T>(spacing: f32, elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column { elements, spacing }
}

pub fn row<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn row_spaced<T>(spacing: f32, elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row { elements, spacing }
}

pub fn stack<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Stack(filter_conditionals(elements))
}

pub fn draw<T>(drawable: T) -> Layout<T> {
    Layout::Draw(Drawable {
        area: Area::default(),
        element: drawable,
    })
}

pub fn conditional<T>(condition: bool, element: Layout<T>) -> Layout<T> {
    Layout::Conditional {
        condition,
        element: Box::new(element),
    }
}

fn filter_conditionals<T>(elements: Vec<Layout<T>>) -> Vec<Layout<T>> {
    elements
        .into_iter()
        .filter_map(|element| {
            if let Layout::Conditional {
                condition,
                element: _,
            } = element
            {
                if condition {
                    element.into()
                } else {
                    None
                }
            } else {
                element.into()
            }
        })
        .collect()
}
