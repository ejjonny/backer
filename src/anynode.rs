use crate::{constraints::SizeConstraints, models::Area};
use std::{any::Any, fmt, rc::Rc};

type AnyDrawFn<State> = Rc<dyn Fn(&dyn Any, &mut State)>;
pub(crate) struct AnyNode<State> {
    pub(crate) inner: Box<dyn Any>,
    pub(crate) clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
    pub(crate) layout: fn(&mut dyn Any, Area),
    pub(crate) sizes: fn(&dyn Any) -> SizeConstraints,
    pub(crate) draw: AnyDrawFn<State>,
}

impl<State> AnyNode<State> {
    pub(crate) fn draw(&self, state: &mut State) {
        (self.draw)(&*self.inner, state)
    }

    pub(crate) fn layout(&mut self, available_area: Area) {
        (self.layout)(&mut *self.inner, available_area)
    }

    pub(crate) fn sizes(&self) -> SizeConstraints {
        (self.sizes)(&*self.inner)
    }
}

impl<State> Clone for AnyNode<State> {
    fn clone(&self) -> Self {
        AnyNode {
            inner: (self.clone)(&self.inner),
            clone: self.clone,
            layout: self.layout,
            sizes: self.sizes,
            draw: self.draw.clone(),
        }
    }
}

impl<State> fmt::Debug for AnyNode<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyNode")
    }
}
