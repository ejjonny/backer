use crate::models::Area;
use std::{fmt, rc::Rc};

type DrawFn<A, B> = Rc<dyn Fn(Area, &'_ mut A, &'_ mut B)>;

#[derive(Clone)]
pub(crate) struct Drawable<A, B> {
    pub(crate) area: Area,
    pub(crate) draw: DrawFn<A, B>,
}

impl<A, B> Drawable<A, B> {
    pub(crate) fn draw(&self, area: Area, a: &mut A, b: &mut B) {
        if area.width > 0. && area.height > 0. {
            (self.draw)(area, a, b);
        }
    }
}

impl<A, B> fmt::Debug for Drawable<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
