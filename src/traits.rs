use crate::{constraints::SizeConstraints, models::Area};

/// Implement `Scopable` to enable usage with [`Node::scope`]
pub trait Scopable<State> {
    /// Provide a scoped mutable reference to a subset of your state.
    ///
    /// This method is called by backer for various purposes,
    /// passing different closures for `f` & using the result returned by `Scopable::scope`.
    ///
    /// ```rust
    /// use backer::nodes::Scopable;
    ///
    /// struct A {
    ///     b: B,
    /// }
    ///
    /// struct B;
    ///
    /// impl Scopable<B> for A {
    ///     fn scope<F, R>(&mut self, f: F) -> R
    ///     where
    ///         F: FnOnce(&mut B) -> R,
    ///     {
    ///        let scoped = &mut self.b;
    ///        f(scoped)
    ///     }
    /// }
    /// ```
    fn scope<F, Result>(&mut self, f: F) -> Result
    where
        F: FnOnce(&mut State) -> Result;
}

pub(crate) trait NodeTrait<State> {
    fn draw(&mut self, state: &mut State);
    fn layout(&mut self, available_area: Area, state: &mut State);
    fn constraints(&mut self, area: Area, state: &mut State) -> SizeConstraints;
}
