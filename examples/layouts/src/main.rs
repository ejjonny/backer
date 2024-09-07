use backer::layout::Layout;
use backer::layout::Node;
use backer::models::*;
use backer::nodes::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HighlightedCase {
    RelAbsSequence,
    AlignmentOffset,
    None,
}

struct State {
    highlight: HighlightedCase,
}

#[macroquad::main("Demo")]
async fn main() {
    let mut state = State {
        highlight: HighlightedCase::None,
    };
    let layout = Layout {
        tree: layout_for_highlight,
    };
    loop {
        layout.draw(
            Area {
                x: 0.,
                y: 0.,
                width: screen_width(),
                height: screen_height(),
            },
            &mut state,
        );
        next_frame().await
    }
}

fn layout_for_highlight(ctx: &State) -> Node<State> {
    let button_size = 50.;
    let highlight = ctx.highlight;
    row_spaced(
        20.,
        vec![
            conditional(
                highlight == HighlightedCase::RelAbsSequence || highlight == HighlightedCase::None,
                column_spaced(
                    10.,
                    vec![
                        scope(|state: &mut State| &mut state.highlight, rect(RED)),
                        text("Mixed (rel/abs) Sequence Constraints", 15., WHITE)
                            .size(Size::new().height(20.)),
                        group(
                            (0..20)
                                .map(|_| rect(BLUE).size(Size::new().height(20.)))
                                .collect(),
                        ),
                        space(),
                        // stack(vec![
                        //     rect(BLUE),
                        //     column_spaced(
                        //         10.,
                        //         vec![
                        //             rect(WHITE),
                        //             rect(WHITE).size(Size::new().height(30.)),
                        //             rect(WHITE),
                        //         ],
                        //     )
                        //     .pad(10.),
                        // ]),
                        button("Fullscreen", |ctx: &mut State| {
                            if ctx.highlight == HighlightedCase::RelAbsSequence {
                                ctx.highlight = HighlightedCase::None;
                            } else {
                                ctx.highlight = HighlightedCase::RelAbsSequence;
                            }
                        })
                        .size(Size::new().height(button_size).y_align(YAlign::Bottom)),
                    ],
                ),
            ),
            conditional(
                highlight == HighlightedCase::AlignmentOffset || highlight == HighlightedCase::None,
                column(vec![
                    text("Alignment & Offset", 15., WHITE).size(Size::new().height(20.)),
                    stack(vec![
                        rect(BLUE),
                        rect(WHITE)
                            .size(Size::new().height(30.).width(30.).x_align(XAlign::Leading)),
                        rect(WHITE)
                            .size(Size::new().height(30.).width(30.).x_align(XAlign::Trailing)),
                        rect(WHITE).size(Size::new().height(30.).width(30.).y_align(YAlign::Top)),
                        rect(WHITE)
                            .size(Size::new().height(30.).width(30.).y_align(YAlign::Bottom)),
                        rect(WHITE)
                            .size(Size::new().height(30.).width(30.).align(Align::TopLeading)),
                        rect(WHITE).size(
                            Size::new()
                                .height(30.)
                                .width(30.)
                                .align(Align::BottomLeading),
                        ),
                        rect(WHITE).size(
                            Size::new()
                                .height(30.)
                                .width(30.)
                                .align(Align::BottomTrailing),
                        ),
                        rect(WHITE)
                            .size(Size::new().height(30.).width(30.).align(Align::TopTrailing)),
                        rect(WHITE)
                            .size(
                                Size::new()
                                    .height(30.)
                                    .width(30.)
                                    .align(Align::CenterCenter),
                            )
                            .offset(10., 10.),
                        rect(WHITE)
                            .size(
                                Size::new()
                                    .height(30.)
                                    .width(30.)
                                    .align(Align::CenterCenter),
                            )
                            .offset(-10., -10.),
                    ]),
                    button("Fullscreen", |ctx: &mut State| {
                        if ctx.highlight == HighlightedCase::AlignmentOffset {
                            ctx.highlight = HighlightedCase::None;
                        } else {
                            ctx.highlight = HighlightedCase::AlignmentOffset;
                        }
                    })
                    .size(Size::new().height(button_size).y_align(YAlign::Bottom)),
                ]),
            ),
        ],
    )
}

fn text<U>(string: &'static str, font_size: f32, color: Color) -> Node<U> {
    let dimensions = measure_text(string, None, font_size as u16, 1.0);
    draw(move |area: Area, _| {
        draw_text(
            string,
            area.x + ((area.width - dimensions.width) * 0.5),
            area.y + (area.height * 0.5) + (dimensions.height * 0.5),
            font_size,
            color,
        );
    })
    .size(
        Size::new()
            .width(dimensions.width)
            .height(dimensions.height),
    )
}

fn rect<U>(color: Color) -> Node<U> {
    draw(move |area: Area, _| {
        draw_rectangle(area.x, area.y, area.width, area.height, color);
    })
}

fn button<U, Action>(label: &'static str, action: Action) -> Node<U>
where
    Action: Fn(&mut U) + 'static,
{
    draw(move |area: Area, ctx| {
        if widgets::Button::new(label)
            .size(vec2(area.width, area.height))
            .position(vec2(area.x, area.y))
            .ui(&mut root_ui())
        {
            action(ctx);
        }
    })
}

// impl FloatRepresentable for HighlightedCase {
//     fn float_value(&self) -> f32 {
//         match self {
//             HighlightedCase::RelAbsSequence => 2.,
//             HighlightedCase::AlignmentOffset => 3.,
//             HighlightedCase::None => 4.,
//         }
//     }
// }

// struct IVec<T>(Vec<T>)
// where
//     T: Interpolable;

// impl<T> Interpolable for IVec<T>
// where
//     T: Interpolable,
// {
//     fn interpolated(&self, other: Self, ratio: f32) -> Self {
//         IVec(
//             std::iter::zip(self.0.iter(), other.0)
//                 .map(|(a, b)| a.interpolated(b, ratio))
//                 .collect(),
//         )
//     }
// }

// #[derive(Clone, Copy)]
// struct IArea(Area);
// impl Interpolable for IArea {
//     fn interpolated(&self, other: Self, ratio: f32) -> Self {
//         IArea(Area {
//             x: self.0.x.interpolated(other.0.x, ratio),
//             y: self.0.y.interpolated(other.0.y, ratio),
//             width: self.0.width.interpolated(other.0.width, ratio),
//             height: self.0.height.interpolated(other.0.height, ratio),
//         })
//     }
// }
