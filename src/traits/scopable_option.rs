use super::Scopable;

/// Implement `ScopableOption` to enable usage with [`scope`] for an optional subset of your state.
/// For non-optional state, implement [`Scopable`].
///
/// See [`Scopable`] for more example code.
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
/// struct Scoper;
/// impl ScopableOption<A, B> for Scoper {
///     fn scope_option<Result>(scoping: &mut A, f: impl FnOnce(Option<&mut B>) -> Result) -> Result {
///        f(scoping.b.as_mut())
///     }
/// }
/// ```
pub trait ScopableOption<'a, Scoping, Scoped> {
    /// Provide a scoped mutable reference to an optional subset of your state.
    fn scope_option<Result>(
        scoping: &'a mut Scoping,
        f: impl FnOnce(Option<&'_ mut Scoped>) -> Result + 'a,
    ) -> Result
    where
        Scoped: 'a;
    // Scoping: 'a;
}

impl<'a, T, Scoping, Scoped> ScopableOption<'a, Scoping, Scoped> for T
where
    T: Scopable<'a, Scoping, Scoped>,
{
    fn scope_option<Result>(
        scoping: &'a mut Scoping,
        f: impl FnOnce(Option<&'_ mut Scoped>) -> Result + 'a,
    ) -> Result
    where
        Scoped: 'a,
        // Scoping: 'a,
    {
        Self::scope(scoping, |s| f(Some(s)))
    }
}
