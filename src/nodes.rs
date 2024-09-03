use crate::{
    layout::{Drawable, Layout},
    models::*,
};

pub fn column<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn column_spaced<T>(spacing: f32, elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing,
    }
}

pub fn row<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn row_spaced<T>(spacing: f32, elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing,
    }
}

pub fn stack<T>(elements: Vec<Layout<T>>) -> Layout<T> {
    Layout::Stack(filter_conditionals(elements))
}

pub fn draw<'a, T, F>(drawable: F) -> Layout<'a, T>
where
    F: FnMut(Area, &mut T) + 'a,
{
    Layout::Draw(Drawable {
        area: Area::default(),
        draw: Some(Box::new(drawable)),
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
