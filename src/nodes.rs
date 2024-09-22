use crate::{
    anynode::AnyNode, constraints::SizeConstraints, drawable::Drawable, layout::NodeValue,
    models::*, Node,
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
/// Return nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader<U>(func: impl Fn(Area, &mut U) -> Node<U> + 'static) -> Node<U> {
    Node {
        inner: NodeValue::AreaReader {
            read: Rc::new(func),
        },
    }
}
/// Narrows or scopes the mutable state available to the children of this node
pub fn scope<U: 'static, V: 'static>(scope: fn(&mut U) -> &mut V, node: Node<V>) -> Node<U> {
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
                    layout: Rc::new(move |any, area, state| {
                        any.downcast_mut::<Node<V>>()
                            .expect("Invalid downcast")
                            .inner
                            .layout(area, None, None, scope(state))
                    }),
                    draw: Rc::new(move |any, state| {
                        any.downcast_ref::<Node<V>>()
                            .expect("Invalid downcast")
                            .inner
                            .draw(scope(state))
                    }),
                    constraints: Rc::new(move |any, area, state| {
                        let scoped = any
                            .downcast_mut::<Node<V>>()
                            .expect("Invalid downcast")
                            .inner
                            .constraints(area, scope(state));
                        SizeConstraints {
                            width: scoped.width,
                            height: scoped.height,
                            aspect: scoped.aspect,
                            dynamic_height: if let Some(dynamic_height) = scoped.dynamic_height {
                                Some(Rc::new(move |w, state| dynamic_height(w, scope(state))))
                            } else {
                                None
                            },
                            dynamic_width: if let Some(dynamic_width) = scoped.dynamic_width {
                                Some(Rc::new(move |h, state| dynamic_width(h, scope(state))))
                            } else {
                                None
                            },
                        }
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
