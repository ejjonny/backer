#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
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
        impl Scopable<B> for A {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut B) -> Result,
            {
                f(&mut self.b)
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
                scope(|b: &mut B| {
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

        impl Scopable<A> for A {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut A) -> Result,
            {
                f(self)
            }
        }

        impl Scopable<C> for B {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut C) -> Result,
            {
                f(&mut self.c)
            }
        }

        fn layout(_: &mut A, _: &mut B) -> NodeWith<A, B> {
            stack(vec![
                draw_with(|area, _, _| {
                    assert_eq!(area, Area::new(0., 0., 100., 100.));
                }),
                scope_with(|_, _| {
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
        fn path_b(_: &mut A) -> Node<A> {
            impl Scopable<B> for A {
                fn scope<F, Result>(&mut self, f: F) -> Result
                where
                    F: FnOnce(&mut B) -> Result,
                {
                    f(&mut self.b)
                }
            }
            stack(vec![scope(|_b: &mut B| space())])
        }
        fn path_c(_: &mut A) -> Node<A> {
            impl Scopable<C> for A {
                fn scope<F, Result>(&mut self, f: F) -> Result
                where
                    F: FnOnce(&mut C) -> Result,
                {
                    f(&mut self.c)
                }
            }
            stack(vec![scope(|_c: &mut C| space())])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: B, c: C });
    }
    #[test]
    fn test_scope_unwrap() {
        struct B;
        struct A {
            b: Option<B>,
        }
        impl ScopableOption<B> for A {
            fn scope_option<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(Option<&mut B>) -> Result,
            {
                f(self.b.as_mut())
            }
        }
        fn layout(_a: &mut A) -> Node<A> {
            stack(vec![scope(|_b: &mut B| space())])
        }
        Layout::new(layout).draw(Area::new(0., 0., 100., 100.), &mut A { b: Some(B) });
    }
    #[test]
    fn test_scope_unwrap_ctx() {
        struct B;
        struct A {
            b: Option<B>,
        }
        impl ScopableOption<B> for A {
            fn scope_option<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(Option<&mut B>) -> Result,
            {
                f(self.b.as_mut())
            }
        }
        impl Scopable<B> for B {
            fn scope<F, Result>(&mut self, f: F) -> Result
            where
                F: FnOnce(&mut B) -> Result,
            {
                f(self)
            }
        }
        fn layout(_b: &mut B, _a: &mut A) -> NodeWith<B, A> {
            stack(vec![scope_with(|_b: &mut B, _b_1: &mut B| space())])
        }
        Layout::new_with(layout).draw_with(
            Area::new(0., 0., 100., 100.),
            &mut B,
            &mut A { b: Some(B) },
        );
    }
}
