use std::fmt::Debug;

use crate::{constraints::SizeConstraints, models::Area};

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

/// Implement `ScopableOption` to enable usage with [`Node::scope`] for optional state.
/// For non-optional state, implement [`Scopable`].
pub trait ScopableOption<Scoped> {
    /// Provide a scoped mutable reference to an optional subset of your state.
    ///
    /// ```rust
    /// use backer::traits::ScopableOption;
    ///
    /// struct A {
    ///     b: Option<B>,
    /// }
    ///
    /// struct B;
    ///
    /// impl ScopableOption<B> for A {
    ///     fn scope_option<F, Result>(&mut self, f: F) -> Result
    ///     where
    ///         F: FnOnce(Option<&mut B>) -> Result,
    ///     {
    ///        f(self.b.as_mut())
    ///     }
    /// }
    /// ```
    fn scope_option<F, Result>(&mut self, f: F) -> Result
    where
        F: FnOnce(Option<&mut Scoped>) -> Result;
}

impl<T, Scoped> ScopableOption<Scoped> for T
where
    T: Scopable<Scoped>,
{
    fn scope_option<F, Result>(&mut self, f: F) -> Result
    where
        F: FnOnce(Option<&mut Scoped>) -> Result,
    {
        self.scope(|s| f(Some(s)))
    }
}

pub(crate) trait NodeTrait<State, Ctx>: Debug {
    fn draw(&mut self, state: &mut State, ctx: &mut Ctx);
    fn layout(&mut self, available_area: Area, state: &mut State, ctx: &mut Ctx);
    fn constraints(&mut self, area: Area, state: &mut State, ctx: &mut Ctx) -> SizeConstraints;
}

impl Scopable<()> for () {
    fn scope<F, Result>(&mut self, f: F) -> Result
    where
        F: FnOnce(&mut ()) -> Result,
    {
        f(self)
    }
}
