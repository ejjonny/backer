use crate::{
    anynode::AnyNode,
    drawable::Drawable,
    layout::{Node, NodeValue},
    models::*,
};
use std::{any::Any, rc::Rc};

pub fn column<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
        },
    }
}

pub fn group<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Group(filter_empty(ungroup(elements))),
    }
}

pub fn column_spaced<U>(spacing: f32, elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing,
        },
    }
}

pub fn row<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
        },
    }
}

pub fn row_spaced<U>(spacing: f32, elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing,
        },
    }
}

pub fn stack<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Stack(filter_empty(ungroup(elements))),
    }
}

pub fn draw<U>(drawable: impl Fn(Area, &mut U) + 'static) -> Node<U> {
    Node {
        inner: NodeValue::Draw(Drawable {
            area: Area::default(),
            draw: Rc::new(drawable),
        }),
    }
}

pub fn space<U>() -> Node<U> {
    Node {
        inner: NodeValue::Space,
    }
}

pub fn logic<U>(element: impl Fn() -> Node<U>) -> Node<U> {
    element()
}

pub fn empty<U>() -> Node<U> {
    Node {
        inner: NodeValue::Empty,
    }
}

pub fn scope<U, V: 'static>(scope: impl Fn(&mut U) -> &mut V + 'static, node: Node<V>) -> Node<U> {
    Node {
        inner: match node.inner {
            NodeValue::Empty => empty().inner,
            _ => NodeValue::<U>::Scope {
                scoped: AnyNode {
                    inner: Box::new(node),
                    clone: |any| {
                        Box::new(
                            any.downcast_ref::<NodeValue<V>>()
                                .expect("Invalid downcast")
                                .clone(),
                        ) as Box<dyn Any>
                    },
                    layout: |any, area| {
                        any.downcast_mut::<NodeValue<V>>()
                            .expect("Invalid downcast")
                            .layout(area)
                    },
                    draw: Rc::new(move |any, state| {
                        any.downcast_ref::<NodeValue<V>>()
                            .expect("Invalid downcast")
                            .draw(scope(state))
                    }),
                },
            },
        },
    }
}

fn ungroup<U>(elements: Vec<Node<U>>) -> Vec<NodeValue<U>> {
    elements
        .into_iter()
        .flat_map(|el| {
            if let NodeValue::Group(els) = el.inner {
                els
            } else {
                vec![el.inner]
            }
        })
        .collect()
}

fn filter_empty<U>(elements: Vec<NodeValue<U>>) -> Vec<NodeValue<U>> {
    elements
        .into_iter()
        .filter(|el| {
            if let NodeValue::Empty = el {
                return false;
            }
            true
        })
        .collect()
}
