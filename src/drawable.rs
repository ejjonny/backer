use crate::{models::Area, traits::Drawable};
use std::fmt;

type DrawFn<State> = Box<dyn Fn(Area, &mut State)>;

pub(crate) enum SomeDrawable<State> {
    Fn(DrawFn<State>),
    Object(Box<dyn Drawable<State>>),
}

impl<State> SomeDrawable<State> {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool) {
        match self {
            SomeDrawable::Fn(closure) => closure(area, state),
            SomeDrawable::Object(object) => object.draw(area, state, visible),
        }
    }
}

pub(crate) struct DrawableNode<State> {
    pub(crate) area: Area,
    pub(crate) drawable: SomeDrawable<State>,
}

impl<State> DrawableNode<State> {
    pub(crate) fn draw(&mut self, area: Area, state: &mut State, contextual_visibility: bool) {
        if area.width >= 0. && area.height >= 0. {
            self.drawable.draw(area, state, contextual_visibility);
        }
    }
}

impl<State> fmt::Debug for DrawableNode<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
