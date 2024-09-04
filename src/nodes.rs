use std::marker::PhantomData;

use crate::{
    layout::{Drawable, Layout},
    models::*,
};

pub fn column<T, U>(elements: Vec<Layout<T, U>>) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn column_spaced<T, U>(spacing: f32, elements: Vec<Layout<T, U>>) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Column {
        elements: filter_conditionals(elements),
        spacing,
    }
}

pub fn row<T, U>(elements: Vec<Layout<T, U>>) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing: 0.,
    }
}

pub fn row_spaced<T, U>(spacing: f32, elements: Vec<Layout<T, U>>) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Row {
        elements: filter_conditionals(elements),
        spacing,
    }
}

pub fn stack<T, U>(elements: Vec<Layout<T, U>>) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Stack(filter_conditionals(elements))
}

pub fn draw<T, U>(drawable: T) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Draw(Drawable {
        area: Area::default(),
        draw: drawable,
        t: PhantomData,
    })
}

pub fn conditional<T, U>(condition: bool, element: Layout<T, U>) -> Layout<T, U>
where
    T: Fn(Area, &mut U),
{
    Layout::Conditional {
        condition,
        element: Box::new(element),
    }
}

fn filter_conditionals<T, U>(elements: Vec<Layout<T, U>>) -> Vec<Layout<T, U>>
where
    T: Fn(Area, &mut U),
{
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
