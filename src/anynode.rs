use crate::{constraints::SizeConstraints, models::Area};
use std::{any::Any, fmt, rc::Rc};

type AnyDrawFn<State> = Rc<dyn Fn(&dyn Any, &mut State)>;
type AnyLayoutFn<State> = Rc<dyn Fn(&mut dyn Any, Area, &mut State)>;
pub(crate) struct AnyNode<State> {
    pub(crate) inner: Box<dyn Any>,
    pub(crate) clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
    pub(crate) layout: AnyLayoutFn<State>,
    pub(crate) constraints: fn(&dyn Any, Area) -> SizeConstraints<State>,
    pub(crate) draw: AnyDrawFn<State>,
}

impl<State> AnyNode<State> {
    pub(crate) fn draw(&self, state: &mut State) {
        (self.draw)(&*self.inner, state)
    }

    pub(crate) fn layout(&mut self, available_area: Area, state: &mut State) {
        (self.layout)(&mut *self.inner, available_area, state)
    }

    pub(crate) fn constraints(&self, area: Area) -> SizeConstraints<State> {
        (self.constraints)(&*self.inner, area)
    }
}

impl<State> Clone for AnyNode<State> {
    fn clone(&self) -> Self {
        AnyNode {
            inner: (self.clone)(&self.inner),
            clone: self.clone,
            layout: self.layout.clone(),
            constraints: self.constraints,
            draw: self.draw.clone(),
        }
    }
}

impl<State> fmt::Debug for AnyNode<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyNode")
    }
}
