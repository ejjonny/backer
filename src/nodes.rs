use crate::{
    anynode::AnyNode,
    drawable::Drawable,
    layout::{Node, NodeValue},
    models::*,
};
use std::{any::Any, rc::Rc};

/// Defines a vertical sequence of elements
pub fn column<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines multiple elements at once.
/// Has no impact on layout.
/// Just a convenience for adding a `Vec` of elements to a sequence node inline.
/// ```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
///
/// column::<()>(vec![
///     empty(),
///     group(
///         (0..5)
///             .into_iter()
///             .map(|i| empty())
///             .collect()
///     ),
/// ]);
/// ```
pub fn group<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Group(filter_empty(ungroup(elements))),
    }
}
/// Defines a vertical sequence of elements with the specified spacing between each element.
pub fn column_spaced<U>(spacing: f32, elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines a horizontal sequence of elements
pub fn row<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines a horizontal sequence of elements with the specified spacing between each element.
pub fn row_spaced<U>(spacing: f32, elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines a sequence of elements to be laid out on top of each other.
pub fn stack<U>(elements: Vec<Node<U>>) -> Node<U> {
    Node {
        inner: NodeValue::Stack(filter_empty(ungroup(elements))),
    }
}
/// Defines a node that can be drawn
/// This node is the point of integration with the UI library of your choice.
/// ```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
///
/// struct MyState {}
/// fn my_drawable(state: &mut MyState) -> Node<MyState> {
///  draw(move |area: Area, state: &mut MyState| {
///    // The `area` parameter is the space alotted for your view after layout is calculated
///    // The `state` parameter is *your* mutable state that you pass when you call `draw`.
///    // This closure should draw UI based on the alotted area or update your state so that drawing can be performed later.
///  })
///}
/// ```
pub fn draw<U>(drawable: impl Fn(Area, &mut U) + 'static) -> Node<U> {
    Node {
        inner: NodeValue::Draw(Drawable {
            area: Area::default(),
            draw: Rc::new(drawable),
        }),
    }
}
/// Defines an empty space which is laid out the same as any other node.
pub fn space<U>() -> Node<U> {
    Node {
        inner: NodeValue::Space,
    }
}
/// Nothing! This will not have any impact on layout - useful for conditionally
/// adding elements to a layout in the case where nothing should be added.
pub fn empty<U>() -> Node<U> {
    Node {
        inner: NodeValue::Empty,
    }
}
/// Narrows or scopes the mutable state available to the children of this node
pub fn scope<U, V: 'static>(scope: impl Fn(&mut U) -> &mut V + 'static, node: Node<V>) -> Node<U> {
    Node {
        inner: match node.inner {
            NodeValue::Empty => empty().inner,
            _ => NodeValue::<U>::Scope {
                scoped: AnyNode {
                    inner: Box::new(node),
                    clone: |any| {
                        Box::new(
                            any.downcast_ref::<Node<V>>()
                                .expect("Invalid downcast")
                                .clone(),
                        ) as Box<dyn Any>
                    },
                    layout: |any, area| {
                        any.downcast_mut::<Node<V>>()
                            .expect("Invalid downcast")
                            .inner
                            .layout(area, None, None)
                    },
                    draw: Rc::new(move |any, state| {
                        any.downcast_ref::<Node<V>>()
                            .expect("Invalid downcast")
                            .inner
                            .draw(scope(state))
                    }),
                    constraints: |any| {
                        any.downcast_ref::<Node<V>>()
                            .expect("Invalid downcast")
                            .inner
                            .constraints()
                    },
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
