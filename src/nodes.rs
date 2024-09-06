use std::rc::Rc;

use crate::{
    layout::{Drawable, Node},
    models::*,
};

pub fn column<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node::Column {
        elements: ungroup(elements),
        spacing: 0.,
    }
}

pub fn group<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node::Group(ungroup(elements))
}

pub fn column_spaced<U>(spacing: f32, elements: Vec<Node<U>>) -> Node<U> {
    Node::Column {
        elements: ungroup(elements),
        spacing,
    }
}

pub fn row<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node::Row {
        elements: ungroup(elements),
        spacing: 0.,
    }
}

pub fn row_spaced<U>(spacing: f32, elements: Vec<Node<U>>) -> Node<U> {
    Node::Row {
        elements: ungroup(elements),
        spacing,
    }
}

pub fn stack<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node::Stack(ungroup(elements))
}

pub fn draw<U>(drawable: impl Fn(Area, &mut U) + 'static) -> Node<U> {
    Node::Draw(Drawable {
        area: Area::default(),
        draw: Rc::new(drawable),
    })
}

pub fn space<U>() -> Node<U> {
    Node::Space
}

pub fn conditional<U>(condition: bool, element: Node<U>) -> Node<U> {
    Node::Conditional {
        condition,
        element: Box::new(element),
    }
}

fn ungroup<U>(elements: Vec<Node<U>>) -> Vec<Node<U>> {
    elements
        .into_iter()
        .flat_map(|el| {
            if let Node::Group(els) = el {
                els
            } else {
                vec![el]
            }
        })
        .collect()
}
