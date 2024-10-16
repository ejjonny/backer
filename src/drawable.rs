use crate::models::Area;
use std::{fmt, rc::Rc};

type DrawFn<State, Ctx> = Rc<dyn Fn(Area, &'_ mut State, &'_ mut Ctx)>;

#[derive(Clone)]
pub(crate) struct Drawable<State, Ctx> {
    pub(crate) area: Area,
    pub(crate) draw: DrawFn<State, Ctx>,
}

impl<State, Ctx> Drawable<State, Ctx> {
    pub(crate) fn draw(&self, area: Area, a: &mut State, b: &mut Ctx) {
        if area.width > 0. && area.height > 0. {
            (self.draw)(area, a, b);
        }
    }
}

impl<State, Ctx> fmt::Debug for Drawable<State, Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
