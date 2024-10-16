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
