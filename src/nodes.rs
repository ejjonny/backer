use crate::{
    drawable::Drawable, layout::NodeValue, models::*, subtree::Subtree, traits::Scopable, Node,
    NodeWith,
};
use std::{marker::PhantomData, rc::Rc};

/// Defines a vertical sequence of elements
pub fn column<State, Ctx>(elements: Vec<NodeWith<State, Ctx>>) -> NodeWith<State, Ctx> {
    NodeWith {
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
/// use backer::traits::Scopable;
///
/// column::<(), ()>(vec![
///     empty(),
///     group(
///         (0..5)
///             .into_iter()
///             .map(|i| empty())
///             .collect()
///     ),
/// ]);
/// ```
pub fn group<State, Ctx>(elements: Vec<NodeWith<State, Ctx>>) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Group(filter_empty(ungroup(elements))),
    }
}
/// Defines a vertical sequence of elements with the specified spacing between each element.
pub fn column_spaced<State, Ctx>(
    spacing: f32,
    elements: Vec<NodeWith<State, Ctx>>,
) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines a horizontal sequence of elements
pub fn row<State, Ctx>(elements: Vec<NodeWith<State, Ctx>>) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines a horizontal sequence of elements with the specified spacing between each element.
pub fn row_spaced<State, Ctx>(
    spacing: f32,
    elements: Vec<NodeWith<State, Ctx>>,
) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Defines a sequence of elements to be laid out on top of each other.
pub fn stack<State, Ctx>(elements: Vec<NodeWith<State, Ctx>>) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Stack(filter_empty(ungroup(elements))),
    }
}
/// Defines a node that can be drawn
/// This node is the point of integration with the UI library of your choice.
/// ```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
/// use backer::traits::Scopable;
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
pub fn draw<State>(drawable: impl Fn(Area, &mut State) + 'static) -> Node<State> {
    NodeWith {
        inner: NodeValue::Draw(Drawable {
            area: Area::default(),
            draw: Rc::new(move |area, a, _| drawable(area, a)),
        }),
    }
}
/// Defines a node that can be drawn
pub fn draw_with<State, Ctx>(
    drawable: impl Fn(Area, &mut State, &mut Ctx) + 'static,
) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Draw(Drawable {
            area: Area::default(),
            draw: Rc::new(move |area, a, b| drawable(area, a, b)),
        }),
    }
}
/// Defines an empty space which is laid out the same as any other node.
pub fn space<State, Ctx>() -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Space,
    }
}
/// Nothing! This will not have any impact on layout - useful for conditionally
/// adding elements to a layout in the case where nothing should be added.
pub fn empty<State, Ctx>() -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::Empty,
    }
}
/// Return nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader<State>(
    func: impl Fn(Area, &mut State, &mut ()) -> Node<State> + 'static,
) -> Node<State> {
    NodeWith {
        inner: NodeValue::AreaReader {
            read: Rc::new(func),
        },
    }
}
/// Return nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader_with<State, Ctx>(
    func: impl Fn(Area, &mut State, &mut Ctx) -> NodeWith<State, Ctx> + 'static,
) -> NodeWith<State, Ctx> {
    NodeWith {
        inner: NodeValue::AreaReader {
            read: Rc::new(func),
        },
    }
}
/// Narrows or scopes the mutable state available to the children of this node
pub fn scope<State, ScopedState>(
    node: impl Fn(&mut ScopedState) -> Node<ScopedState> + 'static,
) -> Node<State>
where
    ScopedState: 'static,
    State: Scopable<ScopedState> + 'static,
{
    NodeWith {
        inner: NodeValue::Scope {
            scoped: Box::new(Subtree {
                subtree_fn: Box::new(move |state, _| node(state)),
                stored_tree: None,
                _p: PhantomData,
                _c: PhantomData,
            }),
        },
    }
}
/// Narrows or scopes the mutable state available to the children of this node
pub fn scope_with<State, ScopedState, Ctx, ScopedCtx>(
    node: impl Fn(&mut ScopedState, &mut ScopedCtx) -> NodeWith<ScopedState, ScopedCtx> + 'static,
) -> NodeWith<State, Ctx>
where
    ScopedState: 'static,
    State: Scopable<ScopedState> + 'static,
    ScopedCtx: 'static,
    Ctx: Scopable<ScopedCtx> + 'static,
{
    NodeWith {
        inner: NodeValue::Scope {
            scoped: Box::new(Subtree {
                subtree_fn: Box::new(node),
                stored_tree: None,
                _p: PhantomData,
                _c: PhantomData,
            }),
        },
    }
}

fn ungroup<State, Ctx>(elements: Vec<NodeWith<State, Ctx>>) -> Vec<NodeValue<State, Ctx>> {
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

fn filter_empty<State, Ctx>(elements: Vec<NodeValue<State, Ctx>>) -> Vec<NodeValue<State, Ctx>> {
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
