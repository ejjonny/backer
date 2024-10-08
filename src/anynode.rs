use crate::{constraints::SizeConstraints, models::Area, Node};
use std::{any::Any, fmt, rc::Rc};

type AnyDrawFn<State> = Rc<dyn Fn(&dyn Any, &mut State)>;
pub type AnyLayoutFn<State> = Rc<dyn Fn(Area, &mut State)>;
pub type AnyConstraintFn<State> = Rc<dyn Fn(Area, &mut State) -> SizeConstraints>;

pub struct AnyNode<State> {
    pub inner: Box<dyn Any>,
    pub clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
    pub layout: AnyLayoutFn<State>,
    pub constraints: AnyConstraintFn<State>,
    pub draw: AnyDrawFn<State>,
}

impl<State> AnyNode<State> {
    pub(crate) fn draw(&self, state: &mut State) {
        (self.draw)(&*self.inner, state)
    }

    pub(crate) fn layout(&mut self, available_area: Area, state: &mut State) {
        (self.layout)(available_area, state)
    }

    pub(crate) fn constraints(&mut self, area: Area, state: &mut State) -> SizeConstraints {
        (self.constraints)(area, state)
    }
}

impl<State> Clone for AnyNode<State> {
    fn clone(&self) -> Self {
        AnyNode {
            inner: (self.clone)(&self.inner),
            clone: self.clone,
            layout: self.layout.clone(),
            constraints: self.constraints.clone(),
            draw: self.draw.clone(),
        }
    }
}

impl<State> fmt::Debug for AnyNode<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyNode")
    }
}
