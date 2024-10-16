/// Implement `Scopable` to enable usage with [`Node::scope`]
pub trait Scopable<Scoped> {
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
    fn scope<F, Result>(&mut self, f: F) -> Result
    where
        F: FnOnce(&mut Scoped) -> Result;
}

impl Scopable<()> for () {
    fn scope<F, Result>(&mut self, f: F) -> Result
    where
        F: FnOnce(&mut ()) -> Result,
    {
        f(self)
    }
}
