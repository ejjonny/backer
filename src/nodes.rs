use std::{marker::PhantomData, rc::Rc};

use crate::{
    layout::{Drawable, Layout},
    models::*,
};

pub fn column<U>(elements: Vec<Layout<U>>) -> Layout<U> {
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn column_spaced<U>(spacing: f32, elements: Vec<Layout<U>>) -> Layout<U> {
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing,
    }
}

pub fn row<U>(elements: Vec<Layout<U>>) -> Layout<U> {
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn row_spaced<U>(spacing: f32, elements: Vec<Layout<U>>) -> Layout<U> {
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing,
    }
}

pub fn stack<U>(elements: Vec<Layout<U>>) -> Layout<U> {
    Layout::Stack(filter_conditionals(elements))
}

pub fn draw<U>(drawable: impl Fn(Area, &mut U) + 'static) -> Layout<U> {
    Layout::Draw(Drawable {
        area: Area::default(),
        draw: Rc::new(drawable),
    })
}

pub fn conditional<U>(condition: bool, element: Layout<U>) -> Layout<U> {
    Layout::Conditional {
        condition,
        element: Box::new(element),
    }
}

fn filter_conditionals<U>(elements: Vec<Layout<U>>) -> Vec<Layout<U>> {
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
