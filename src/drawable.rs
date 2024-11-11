use crate::{models::Area, traits::Drawable};
use std::fmt;

impl<State, F: Fn(Area, &mut State)> Drawable<State> for F {
    fn draw(&mut self, area: Area, state: &mut State, _visible: bool) {
        self(area, state)
    }
}

pub(crate) struct DrawableNode<State> {
    pub(crate) area: Area,
    pub(crate) drawable: Box<dyn Drawable<State>>,
    pub(crate) visible: bool,
}

impl<State> DrawableNode<State> {
    pub(crate) fn draw(&mut self, area: Area, state: &mut State, contextual_visibility: bool) {
        if area.width >= 0. && area.height >= 0. {
            self.drawable
                .draw(area, state, contextual_visibility && self.visible);
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
