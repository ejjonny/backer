use std::marker::PhantomData;

/// Implement `Scopable` to enable usage with [`Node::scope`]
pub trait Scopable<Scoping, Scoped> {
    /// Provide a scoped mutable reference to a subset of your state.
    ///
    /// This method is called by backer for various purposes,
    /// passing different closures for `f` & using the result returned by `Scopable::scope`.
    ///
    /// ```rust
    ///
    /// use backer::traits::Scopable;
    ///
    /// struct A {
    ///     b: B,
    /// }
    ///
    /// struct B;
    ///
    /// impl Scopable<B> for A {
    ///     fn scope<F, Result>(&mut self, f: F) -> Result
    ///     where
    ///         F: FnOnce(&mut B) -> Result,
    ///     {
    ///        let scoped = &mut self.b;
    ///        f(scoped)
    ///     }
    /// }
    /// ```
    fn scope<F, Result>(scoping: &mut Scoping, f: F) -> Result
    where
        F: FnOnce(&mut Scoped) -> Result;
}

pub(crate) struct VoidScoper;

impl Scopable<(), ()> for VoidScoper {
    fn scope<F, Result>(scoping: &mut (), f: F) -> Result
    where
        F: FnOnce(&mut ()) -> Result,
    {
        f(scoping)
    }
}

pub struct NoOpScoper<Scoping> {
    _s: PhantomData<Scoping>,
}

impl<Scoping> Scopable<Scoping, Scoping> for NoOpScoper<Scoping> {
    fn scope<F, Result>(scoping: &mut Scoping, f: F) -> Result
    where
        F: FnOnce(&mut Scoping) -> Result,
    {
        f(scoping)
    }
}
