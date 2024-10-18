#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    // use crate::traits::NoOpScoper;
    use crate::traits::Scopable;
    use crate::Node;
    // #[test]
    // fn test_scope() {
    //     struct A {
    //         test: bool,
    //         b: B,
    //     }
    //     struct B {
    //         test: bool,
    //     }
    //     let mut a = A {
    //         test: true,
    //         b: B { test: true },
    //     };
    //     struct AToBScoper;
    //     impl Scopable<A, B> for AToBScoper {
    //         fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut B) -> Result) -> Result {
    //             f(&mut scoping.b)
    //         }
    //     }
    //     fn layout<'a>(a: &mut A) -> Node<'a, A> {
    //         stack(vec![
    //             if a.test {
    //                 draw(|area, a: &mut A| {
    //                     assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                     a.test = false;
    //                 })
    //             } else {
    //                 draw(|area, a: &mut A| {
    //                     assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                     a.test = true;
    //                 })
    //             },
    //             scope::<_, _, AToBScoper>(|b: &mut B| {
    //                 if b.test {
    //                     draw(|area, b: &mut B| {
    //                         assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                         b.test = false;
    //                     })
    //                 } else {
    //                     draw(|area, b: &mut B| {
    //                         assert_eq!(area, Area::new(0., 0., 100., 100.));
    //                         b.test = true;
    //                     })
    //                 }
    //             }),
    //         ])
    //     }
    //     Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
    //     assert!(!a.test);
    //     assert!(!a.b.test);
    //     Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
    //     assert!(a.test);
    //     assert!(a.b.test);
    // }
    #[test]
    fn test_partial_scope_variadic() {
        struct A;
        struct C;
        struct B {
            c: C,
        }
        struct PartialScoper;

        // impl PartialScoper {
        impl<'state> Scopable<'state, State<'state>, ScopedState<'state>> for PartialScoper {
            fn scope<Result>(
                scoping: &'state mut State<'state>,
                f: impl FnOnce(&'_ mut ScopedState<'state>) -> Result + 'state,
            ) -> Result
            where
                ScopedState<'state>: 'state,
                State<'state>: 'state,
            {
                let mut scoped = ScopedState(&mut *scoping.0, &mut scoping.1.c);
                f(&mut scoped)
            }

            // fn scopu<'scoping, 'scoped, Result>(
            //     scoping: &'scoping mut State<'scoping>,
            //     f: impl FnOnce(&'_ mut ScopedState<'scoping>) -> Result + 'scoped,
            // ) -> Result
            // where
            //     ScopedState<'scoping>: 'scoped,
            //     State<'scoping>: 'scoping,
            // {
            //     let mut scoped = ScopedState(&mut *scoping.0, &mut scoping.1.c);
            //     f(&mut scoped)
            // }
        }

        // impl<'state> PartialScoper {
        // impl<'state> Scopable<State<'state>, ScopedState<'state>> for PartialScoper {
        //     fn scope<'scoping, 'scoped, Result>(
        //         scoping: &'scoping mut State<'state>,
        //         f: impl FnOnce(&'_ mut ScopedState<'state>) -> Result + 'scoped,
        //     ) -> Result
        //     where
        //         ScopedState<'state>: 'scoped,
        //         State<'state>: 'scoping,
        //     {
        //         let mut scoped = ScopedState(&mut *scoping.0, &mut scoping.1.c);
        //         f(&mut scoped)
        //     }

        //works
        // fn scopee<'scoping, 'scoped, Result>(
        //     scoping: &'scoping mut State<'state>,
        //     f: impl FnOnce(&'_ mut ScopedState<'state>) -> Result + 'scoped,
        // ) -> Result
        // where
        //     'scoping: 'state,
        // {
        //     let mut scoped = ScopedState(&mut *scoping.0, &mut scoping.1.c);
        //     f(&mut scoped)
        // }
        // }

        fn layout<'nodes, 'state>(_: &mut State) -> Node<'nodes, State<'state>> {
            stack(vec![
                draw(|area, _| {
                    assert_eq!(area, Area::new(0., 0., 100., 100.));
                }),
                // scope::<_, _, PartialScoper>(|scoped| {
                //     draw(|area, _a: &mut ScopedState| {
                //         assert_eq!(area, Area::new(0., 0., 100., 100.));
                //     })
                // }),
            ])
        }
        struct State<'a>(&'a mut A, &'a mut B);
        let mut a = A;
        let mut b = B { c: C };
        struct ScopedState<'a>(&'a mut A, &'a mut C);
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut State(&mut a, &mut b));
    }
    // #[test]
    // fn test_multiple_scope_paths() {
    //     struct C;
    //     struct B;
    //     struct A {
    //         b: B,
    //         c: C,
    //     }
    //     fn layout<'a, 'b>(a: &'b mut A) -> Node<'a, A>
    //     where
    //         'b: 'a,
    //     {
    //         stack(vec![path_b(a), path_c(a)])
    //     }
    //     struct AToBScoper;
    //     impl Scopable<A, B> for AToBScoper {
    //         fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut B) -> Result) -> Result {
    //             f(&mut scoping.b)
    //         }
    //     }
    //     struct AToCScoper;
    //     impl Scopable<A, C> for AToCScoper {
    //         fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut C) -> Result) -> Result {
    //             f(&mut scoping.c)
    //         }
    //     }
    //     // fn path_b(_: &mut A) -> Node<A> {
    //     fn path_b<'a>(_: &mut A) -> Node<'a, A> {
    //         stack(vec![scope::<_, _, AToBScoper>(|_b: &mut B| space())])
    //     }
    //     fn path_c<'a>(_: &mut A) -> Node<'a, A> {
    //         stack(vec![scope::<_, _, AToCScoper>(|_c: &mut C| space())])
    //     }
    //     Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: B, c: C });
    // }
    // #[test]
    // fn test_scope_unwrap() {
    //     struct B;
    //     struct A {
    //         b: Option<B>,
    //     }
    //     struct AScoper;
    //     impl ScopableOption<A, B> for AScoper {
    //         fn scope_option<Result>(
    //             scoping: &mut A,
    //             f: impl FnOnce(Option<&mut B>) -> Result,
    //         ) -> Result {
    //             f(scoping.b.as_mut())
    //         }
    //     }
    //     fn layout<'a>(_a: &'_ mut A) -> Node<'a, A> {
    //         stack(vec![scope::<_, _, AScoper>(|_b: &mut B| space())])
    //     }
    //     Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: Some(B) });
    // }
    // #[test]
    // fn test_scope_unwrap_ctx() {
    //     struct B;
    //     struct A {
    //         b: Option<B>,
    //     }
    //     struct AScoper;
    //     impl ScopableOption<A, B> for AScoper {
    //         fn scope_option<Result>(
    //             scoping: &mut A,
    //             f: impl FnOnce(Option<&mut B>) -> Result,
    //         ) -> Result {
    //             f(scoping.b.as_mut())
    //         }
    //     }
    //     struct BScoper;
    //     impl Scopable<B, B> for BScoper {
    //         fn scope<Result>(scoping: &mut B, f: impl FnOnce(&mut B) -> Result) -> Result {
    //             f(scoping)
    //         }
    //     }
    //     fn layout(_b: &mut B, _a: &mut A) -> Node<B, A> {
    //         stack(vec![scope::<_, _, _, _, BScoper, AScoper>(
    //             |_b: &mut B, _b_1: &mut B| space(),
    //         )])
    //     }
    //     Layout::new_with(layout).draw_with(
    //         Area::new(0., 0., 100., 100.),
    //         &mut B,
    //         &mut A { b: Some(B) },
    //     );
    // }
    // #[test]
    // fn test_scope_invariant() {
    //     struct A;
    //     struct B<'a> {
    //         a: &'a mut A,
    //     }
    //     struct Scoper;
    //     impl<'a> Scopable<B<'a>, A> for Scoper {
    //         fn scope<Result>(scoping: &mut B, f: impl FnOnce(&mut A) -> Result) -> Result {
    //             f(scoping.a)
    //         }
    //     }
    //     fn layout<'a>(_b: &mut B<'a>) -> Node<'a, B<'a>> {
    //         scope::<'a, _, _, Scoper>(layout_scoped)
    //     }
    //     fn layout_scoped<'a>(_b: &mut A) -> Node<'a, A> {
    //         draw(|_, _: &mut A| {})
    //     }
    //     let mut a = A;
    //     Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut B { a: &mut a });
    // }
}
