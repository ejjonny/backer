use crate::{
    anynode::AnyNode, constraints::SizeConstraints, drawable::Drawable, layout::NodeValue,
    models::*, Node,
};
use std::{any::Any, rc::Rc};

/// Defines a vertical sequence of elements
pub fn column<A, B>(elements: Vec<Node<A, B>>) -> Node<A, B> {
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
pub fn group<A, B>(elements: Vec<Node<A, B>>) -> Node<A, B> {
    Node {
        inner: NodeValue::Group(filter_empty(ungroup(elements))),
    }
}
/// Defines a vertical sequence of elements with the specified spacing between each element.
pub fn column_spaced<A, B>(spacing: f32, elements: Vec<Node<A, B>>) -> Node<A, B> {
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
pub fn row<A, B>(elements: Vec<Node<A, B>>) -> Node<A, B> {
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
pub fn row_spaced<A, B>(spacing: f32, elements: Vec<Node<A, B>>) -> Node<A, B> {
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
pub fn stack<A, B>(elements: Vec<Node<A, B>>) -> Node<A, B> {
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
pub fn draw<A>(drawable: impl Fn(Area, &mut A) + 'static) -> Node<A, ()> {
    Node {
        inner: NodeValue::Draw(Drawable {
            area: Area::default(),
            draw: Rc::new(move |area, a, _| drawable(area, a)),
        }),
    }
}
/// Defines an empty space which is laid out the same as any other node.
pub fn space<A, B>() -> Node<A, B> {
    Node {
        inner: NodeValue::Space,
    }
}
/// Nothing! This will not have any impact on layout - useful for conditionally
/// adding elements to a layout in the case where nothing should be added.
pub fn empty<A, B>() -> Node<A, B> {
    Node {
        inner: NodeValue::Empty,
    }
}
/// Return nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader<A, B>(
    func: impl Fn(Area, &mut A, &mut B) -> Node<A, B> + 'static,
) -> Node<A, B> {
    Node {
        inner: NodeValue::AreaReader {
            read: Rc::new(func),
        },
    }
}
/// Narrows or scopes the mutable state available to the children of this node
pub fn scope_with<T, U, A: 'static, B: 'static>(
    scope_a: impl Fn(&mut T) -> &mut A + 'static + Copy,
    scope_b: impl Fn(&mut U) -> &mut B + 'static + Copy,
    node: impl Fn(&mut A, &mut B) -> Node<A, B> + 'static + Copy,
) -> Node<T, U> {
    Node {
        inner: NodeValue::Scope {
            node: None,
            scope_a: Rc::new(move |a| scope_a(a)),
            scope_b: Rc::new(move |b| scope_b(b)),
            scoped: Rc::new(move |any_a, any_b| {
                let downcast_a = any_a.downcast_mut::<&mut A>().expect("Invalid downcast");
                let downcast_b = any_b.downcast_mut::<&mut B>().expect("Invalid downcast");
                let anynode = node(downcast_a, downcast_b);
                AnyNode {
                    inner: Box::new(anynode),
                    clone: move |any| {
                        Box::new(
                            any.downcast_ref::<Node<A, B>>()
                                .expect("Invalid downcast")
                                .clone(),
                        ) as Box<dyn Any>
                    },
                    layout: Rc::new(move |any, area, a, b| {
                        any.downcast_mut::<Node<A, B>>()
                            .expect("Invalid downcast")
                            .inner
                            .layout(area, None, None, scope_a(a), scope_b(b))
                    }),
                    draw: Rc::new(move |any, a, b| {
                        any.downcast_ref::<Node<A, B>>()
                            .expect("Invalid downcast")
                            .inner
                            .draw(scope_a(a), scope_b(b))
                    }),
                    constraints: Rc::new(move |any, area, a, b| {
                        let scoped = any
                            .downcast_mut::<Node<A, B>>()
                            .expect("Invalid downcast")
                            .inner
                            .constraints(area, scope_a(a), scope_b(b));
                        SizeConstraints {
                            width: scoped.width,
                            height: scoped.height,
                            aspect: scoped.aspect,
                        }
                    }),
                }
            }),
        },
    }
}

/// Narrows or scopes the mutable state available to the children of this node
pub fn scope<T, A: 'static>(
    scope_a: impl Fn(&mut T) -> &mut A + 'static + Copy,
    node: impl Fn(&mut A) -> Node<A, ()> + 'static + Copy,
) -> Node<T, ()> {
    scope_with(scope_a, |b| b, move |a, _| node(a))
}

fn ungroup<A, B>(elements: Vec<Node<A, B>>) -> Vec<NodeValue<A, B>> {
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

fn filter_empty<A, B>(elements: Vec<NodeValue<A, B>>) -> Vec<NodeValue<A, B>> {
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
