#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    #[test]
    fn test_column_basic() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 50.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 50., 100., 50.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_column_constrained_1() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 90.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 90.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_column_constrained_2() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 90.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 100., 10.));
                })
                .height(10.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 90.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 100., 10.));
                })
                .height(10.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_basic() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_constrained_1() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 25., 10., 50.));
                })
                .width(10.)
                .height(50.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 0., 90., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(YAlign::Top),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 40., 10., 20.));
                })
                .width(10.)
                .height(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(20., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(YAlign::Bottom),
                draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 70., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_constrained_2() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 70., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(YAlign::Top),
                draw(|a, _| {
                    assert_eq!(a, Area::new(80., 40., 10., 20.));
                })
                .width(10.)
                .height(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(YAlign::Bottom),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 70., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(YAlign::Top),
                draw(|a, _| {
                    assert_eq!(a, Area::new(80., 40., 10., 20.));
                })
                .width(10.)
                .height(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(YAlign::Bottom),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_stack_basic() {
        Layout::new(|()| {
            stack(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_stack_alignment() {
        Layout::new(|()| {
            stack(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::TopLeading),
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::TopCenter),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 0., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::TopTrailing),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 40., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::CenterTrailing),
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::BottomTrailing),
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::BottomCenter),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 80., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::BottomLeading),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::CenterLeading),
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 40., 10., 20.));
                })
                .width(10.)
                .height(20.)
                .align(Align::CenterCenter),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_sequence_spacing() {
        Layout::new(|()| {
            row_spaced(
                10.,
                vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(20., 0., 25., 100.));
                    }),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(55., 40., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(75., 0., 25., 100.));
                    }),
                ],
            )
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column_spaced(
                10.,
                vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 0., 100., 15.));
                    }),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(45., 25., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 55., 100., 15.));
                    }),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(45., 80., 10., 20.));
                    })
                    .width(10.)
                    .height(20.),
                ],
            )
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_row_with_constrained_item() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 30., 100.));
                })
                .width(30.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 70., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_nested_row_with_constrained_item() {
        Layout::new(|()| {
            row(vec![
                row(vec![
                    draw(|a, _| {
                        assert_eq!(a, Area::new(0., 0., 20., 100.));
                    })
                    .width(20.),
                    draw(|a, _| {
                        assert_eq!(a, Area::new(20., 0., 30., 100.));
                    }),
                ])
                .width(50.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_stack_with_constrained_item() {
        Layout::new(|()| {
            stack(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(25., 25., 50., 50.));
                })
                .width(50.)
                .height(50.),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    #[test]
    fn test_row_with_multiple_constrained_items() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 20., 100.));
                })
                .width(20.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(20., 25., 30., 50.));
                })
                .width(30.)
                .height(50.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 25., 100.));
                }),
                draw(|a, _| {
                    assert_eq!(a, Area::new(75., 0., 25., 100.));
                }),
            ])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }

    // #[test]
    // fn test_constraint_combination() {
    //     assert_eq!(
    //         row::<()>(vec![space(), space().height(30.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint::none(),
    //             height: Constraint {
    //                 lower: Some(30.),
    //                 upper: None
    //             },
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         row::<()>(vec![space().height(40.), space().height(30.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint::none(),
    //             height: Constraint {
    //                 lower: Some(40.),
    //                 upper: Some(40.)
    //             },
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         column::<()>(vec![space(), space().width(10.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint {
    //                 lower: Some(10.),
    //                 upper: None
    //             },
    //             height: Constraint::none(),
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         column::<()>(vec![space().width(20.), space().width(10.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint {
    //                 lower: Some(20.),
    //                 upper: Some(20.)
    //             },
    //             height: Constraint::none(),
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         stack::<()>(vec![space(), space().height(10.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint::none(),
    //             height: Constraint {
    //                 lower: Some(10.),
    //                 upper: None
    //             },
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         stack::<()>(vec![space().height(20.), space().width(10.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint {
    //                 lower: Some(10.),
    //                 upper: None
    //             },
    //             height: Constraint {
    //                 lower: Some(20.),
    //                 upper: None
    //             },
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         stack::<()>(vec![space().height(20.), space().height(10.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint {
    //                 lower: None,
    //                 upper: None
    //             },
    //             height: Constraint {
    //                 lower: Some(20.),
    //                 upper: Some(20.)
    //             },
    //             aspect: None
    //         }
    //     );
    //     assert_eq!(
    //         stack::<()>(vec![space().width(20.), space().width(10.)])
    //             .inner
    //             .constraints(Area::zero()),
    //         SizeConstraints {
    //             width: Constraint {
    //                 lower: Some(20.),
    //                 upper: Some(20.)
    //             },
    //             height: Constraint {
    //                 lower: None,
    //                 upper: None
    //             },
    //             aspect: None
    //         }
    //     );
    // }
}
