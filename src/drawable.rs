use crate::models::Area;
use std::{fmt, rc::Rc};

type DrawFn<State> = Rc<dyn Fn(Area, &'_ mut State)>;

#[derive(Clone)]
pub struct Drawable<State> {
    pub area: Area,
    pub(crate) draw: DrawFn<State>,
}

impl<State> Drawable<State> {
    pub fn draw(&self, area: Area, state: &mut State) {
        if area.width > 0. && area.height > 0. {
            (self.draw)(area, state);
        }
    }
}

impl<State> fmt::Debug for Drawable<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
