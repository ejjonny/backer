#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    use crate::traits::NoOpScoper;
    use crate::traits::Scopable;
    use crate::traits::ScopableOption;
    use crate::Node;
    use crate::NodeWith;
    #[test]
    fn test_scope() {
        struct A {
            test: bool,
            b: B,
        }
        struct B {
            test: bool,
        }
        let mut a = A {
            test: true,
            b: B { test: true },
        };
        struct AToBScoper;
        impl Scopable<A, B> for AToBScoper {
            fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut B) -> Result) -> Result {
                f(&mut scoping.b)
            }
        }
        fn layout(a: &mut A) -> NodeWith<A, ()> {
            stack(vec![
                if a.test {
                    draw(|area, a: &mut A| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                        a.test = false;
                    })
                } else {
                    draw(|area, a: &mut A| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                        a.test = true;
                    })
                },
                scope::<_, _, AToBScoper>(|b: &mut B| {
                    if b.test {
                        draw(|area, b: &mut B| {
                            assert_eq!(area, Area::new(0., 0., 100., 100.));
                            b.test = false;
                        })
                    } else {
                        draw(|area, b: &mut B| {
                            assert_eq!(area, Area::new(0., 0., 100., 100.));
                            b.test = true;
                        })
                    }
                }),
            ])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
        assert!(!a.test);
        assert!(!a.b.test);
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut a);
        assert!(a.test);
        assert!(a.b.test);
    }
    #[test]
    fn test_partial_scope_variadic() {
        struct A;
        struct C;
        struct B {
            c: C,
        }
        struct BToCScoper;
        impl Scopable<B, C> for BToCScoper {
            fn scope<Result>(scoping: &mut B, f: impl FnOnce(&mut C) -> Result) -> Result {
                f(&mut scoping.c)
            }
        }

        fn layout(_: &mut A, _: &mut B) -> NodeWith<A, B> {
            stack(vec![
                draw_with(|area, _, _| {
                    assert_eq!(area, Area::new(0., 0., 100., 100.));
                }),
                scope_with::<_, _, _, _, NoOpScoper<A>, BToCScoper>(|_, _| {
                    draw_with(|area, _a: &mut A, _c: &mut C| {
                        assert_eq!(area, Area::new(0., 0., 100., 100.));
                    })
                }),
            ])
        }
        Layout::new_with(layout).draw_with(Area::new(0., 0., 100., 100.), &mut A, &mut B { c: C });
    }
    #[test]
    fn test_multiple_scope_paths() {
        struct C;
        struct B;
        struct A {
            b: B,
            c: C,
        }
        fn layout(a: &mut A) -> Node<A> {
            stack(vec![path_b(a), path_c(a)])
        }
        struct AToBScoper;
        impl Scopable<A, B> for AToBScoper {
            fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut B) -> Result) -> Result {
                f(&mut scoping.b)
            }
        }
        struct AToCScoper;
        impl Scopable<A, C> for AToCScoper {
            fn scope<Result>(scoping: &mut A, f: impl FnOnce(&mut C) -> Result) -> Result {
                f(&mut scoping.c)
            }
        }
        fn path_b(_: &mut A) -> Node<A> {
            stack(vec![scope::<_, _, AToBScoper>(|_b: &mut B| space())])
        }
        fn path_c(_: &mut A) -> Node<A> {
            stack(vec![scope::<_, _, AToCScoper>(|_c: &mut C| space())])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: B, c: C });
    }
    #[test]
    fn test_scope_unwrap() {
        struct B;
        struct A {
            b: Option<B>,
        }
        struct AScoper;
        impl ScopableOption<A, B> for AScoper {
            fn scope_option<Result>(
                scoping: &mut A,
                f: impl FnOnce(Option<&mut B>) -> Result,
            ) -> Result {
                f(scoping.b.as_mut())
            }
        }
        fn layout(_a: &mut A) -> Node<A> {
            stack(vec![scope::<_, _, AScoper>(|_b: &mut B| space())])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: Some(B) });
    }
    #[test]
    fn test_scope_unwrap_ctx() {
        struct B;
        struct A {
            b: Option<B>,
        }
        struct AScoper;
        impl ScopableOption<A, B> for AScoper {
            fn scope_option<Result>(
                scoping: &mut A,
                f: impl FnOnce(Option<&mut B>) -> Result,
            ) -> Result {
                f(scoping.b.as_mut())
            }
        }
        struct BScoper;
        impl Scopable<B, B> for BScoper {
            fn scope<Result>(scoping: &mut B, f: impl FnOnce(&mut B) -> Result) -> Result {
                f(scoping)
            }
        }
        fn layout(_b: &mut B, _a: &mut A) -> NodeWith<B, A> {
            stack(vec![scope_with::<_, _, _, _, BScoper, AScoper>(
                |_b: &mut B, _b_1: &mut B| space(),
            )])
        }
        Layout::new_with(layout).draw_with(
            Area::new(0., 0., 100., 100.),
            &mut B,
            &mut A { b: Some(B) },
        );
    }
}
