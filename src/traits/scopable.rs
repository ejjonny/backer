use std::marker::PhantomData;

/// Implement `Scopable` to enable usage with [`Node::scope`]
///
/// ```rust
/// use backer::traits::*;
/// use backer::nodes::*;
/// use backer::models::*;
/// use backer::{Node, Layout};
///
/// struct A {
///     b: B,
/// }
/// struct B;
///
/// struct Scoper;
/// impl Scopable<A, B> for Scoper {
///     fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut B) -> Result) -> Result {
///         f(&mut scoping.b)
///     }
/// }
///
/// let layout = Layout::new(my_layout);
/// layout.draw(Area::new(0., 0., 100., 100.), &mut A { b: B });
///
/// fn my_layout(_state: &mut A) -> Node<A> {
///     scope::<_, _, Scoper>(my_scoped_layout)
/// }
///
/// // This tree & it's children now only have access to a subset of our state
/// fn my_scoped_layout(_state: &mut B) -> Node<B> {
///     draw(|_area, _state: &mut B| {})
/// }
/// ```
pub trait Scopable<Scoping, Scoped> {
    /// Provide a scoped mutable reference to a subset of your state.
    ///
    /// This method is called by backer for various purposes,
    /// passing different closures for `f` & using the result returned by `Scopable::scope`.
    fn scope<Result>(scoping: &mut Scoping, f: impl FnOnce(&mut Scoped) -> Result) -> Result;
}

pub(crate) struct VoidScoper;

impl Scopable<(), ()> for VoidScoper {
    fn scope<Result>(scoping: &mut (), f: impl FnOnce(&mut ()) -> Result) -> Result {
        f(scoping)
    }
}

/// Used to implement scoping which has no effect.
/// Useful for scoping one of State / Ctx without scoping the other.
pub struct NoOpScoper<Scoping> {
    _s: PhantomData<Scoping>,
}

impl<Scoping> Scopable<Scoping, Scoping> for NoOpScoper<Scoping> {
    fn scope<Result>(scoping: &mut Scoping, f: impl FnOnce(&mut Scoping) -> Result) -> Result {
        f(scoping)
    }
}
