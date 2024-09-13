#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::models::*;
    use crate::nodes::*;
    #[test]
    fn test_seq_align_on_axis() {
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 0., 30., 100.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(40., 0., 30., 100.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(60., 0., 10., 100.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 30., 100.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 30., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 100., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 60., 100., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 70., 100., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_off_axis() {
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 50., 30., 50.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(35., 50., 30., 50.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(90., 0., 10., 50.));
                })
                .width(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 50., 30., 50.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 45., 50., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 35., 50., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 50., 10.));
                })
                .height(10.),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 70., 50., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_on_axis_nested_seq() {
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(10., 0., 30., 100.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(30., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(40., 0., 30., 100.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(60., 0., 10., 100.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 0., 30., 100.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 10., 100., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 30., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 40., 100., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 60., 100., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 70., 100., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_seq_align_off_axis_nested_seq() {
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(0., 50., 30., 50.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(45., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(35., 50., 30., 50.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(90., 0., 10., 50.));
                })
                .width(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(70., 50., 30., 50.));
                })
                .width(30.),
            ])
            .x_align(XAlign::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 0., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 0., 50., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 45., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 35., 50., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Center)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            row(vec![
                row(vec![draw(|a, _| {
                    assert_eq!(a, Area::new(0., 90., 50., 10.));
                })
                .height(10.)]),
                draw(|a, _| {
                    assert_eq!(a, Area::new(50., 70., 50., 30.));
                })
                .height(30.),
            ])
            .y_align(YAlign::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio() {
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 100.));
            })
            .aspect(1.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 50., 100.));
            })
            .aspect(0.5)
            .x_align(XAlign::Leading)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(50., 0., 50., 100.));
            })
            .aspect(0.5)
            .x_align(XAlign::Trailing)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());

        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 25., 100., 50.));
            })
            .aspect(2.)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 50.));
            })
            .aspect(2.)
            .y_align(YAlign::Top)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            draw(|a, _| {
                assert_eq!(a, Area::new(0., 50., 100., 50.));
            })
            .aspect(2.)
            .y_align(YAlign::Bottom)
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
    #[test]
    fn test_aspect_ratio_in_seq() {
        Layout::new(|()| {
            row(vec![draw(|a, _| {
                assert_eq!(a, Area::new(0., 0., 100., 100.));
            })
            .aspect(1.)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            stack(vec![draw(|a, _| {
                assert_eq!(a, Area::new(25., 0., 50., 100.));
            })
            .aspect(0.5)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            column(vec![draw(|a, _| {
                assert_eq!(a, Area::new(0., -50., 100., 200.));
            })
            .aspect(0.5)
            .x_align(XAlign::Leading)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
        Layout::new(|()| {
            stack(vec![draw(|a, _| {
                assert_eq!(a, Area::new(50., 0., 50., 100.));
            })
            .aspect(0.5)
            .x_align(XAlign::Trailing)])
        })
        .draw(Area::new(0., 0., 100., 100.), &mut ());
    }
}
