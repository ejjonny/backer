use crate::{constraints::SizeConstraints, models::Area};
use std::{any::Any, fmt, rc::Rc};

type AnyDrawFn<A, B> = Rc<dyn Fn(&dyn Any, &mut A, &mut B)>;
type AnyLayoutFn<A, B> = Rc<dyn Fn(&mut dyn Any, Area, &mut A, &mut B)>;
type AnyConstraintFn<A, B> = Rc<dyn Fn(&mut dyn Any, Area, &mut A, &mut B) -> SizeConstraints>;
pub(crate) struct AnyNode<A, B> {
    pub(crate) inner: Box<dyn Any>,
    pub(crate) clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
    pub(crate) layout: AnyLayoutFn<A, B>,
    pub(crate) constraints: AnyConstraintFn<A, B>,
    pub(crate) draw: AnyDrawFn<A, B>,
}

impl<A, B> AnyNode<A, B> {
    pub(crate) fn draw(&self, a: &mut A, b: &mut B) {
        (self.draw)(&*self.inner, a, b)
    }

    pub(crate) fn layout(&mut self, available_area: Area, a: &mut A, b: &mut B) {
        (self.layout)(&mut *self.inner, available_area, a, b)
    }

    pub(crate) fn constraints(&mut self, area: Area, a: &mut A, b: &mut B) -> SizeConstraints {
        (self.constraints)(&mut *self.inner, area, a, b)
    }
}

impl<A, B> Clone for AnyNode<A, B> {
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

impl<A, B> fmt::Debug for AnyNode<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyNode")
    }
}
