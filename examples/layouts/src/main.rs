use backer::layout::Layout;
use backer::models::*;
use backer::nodes::*;
use lilt::Animated;
use lilt::FloatRepresentable;
use lilt::Interpolable;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HighlightedCase {
    HeightConstraints,
    WidthConstraints,
    RelAbsSequence,
    AlignmentOffset,
    None,
}

impl FloatRepresentable for HighlightedCase {
    fn float_value(&self) -> f32 {
        match self {
            HighlightedCase::HeightConstraints => 0.,
            HighlightedCase::WidthConstraints => 1.,
            HighlightedCase::RelAbsSequence => 2.,
            HighlightedCase::AlignmentOffset => 3.,
            HighlightedCase::None => 4.,
        }
    }
}

#[derive(Clone, Copy)]
struct IArea(Area);
impl Interpolable for IArea {
    fn interpolated(&self, other: Self, ratio: f32) -> Self {
        IArea(Area {
            x: self.0.x.interpolated(other.0.x, ratio),
            y: self.0.y.interpolated(other.0.y, ratio),
            width: self.0.width.interpolated(other.0.width, ratio),
            height: self.0.height.interpolated(other.0.height, ratio),
        })
    }
}

struct Context<'a> {
    highlight: &'a mut HighlightedCase,
    anim: &'a mut Animated<f32, Instant>,
}

#[macroquad::main("Demo")]
async fn main() {
    let mut show_alignment: HighlightedCase = HighlightedCase::None;
    let mut anim = Animated::<f32, Instant>::new(0.);
    let mut last_drawables = Option::<Vec<Area>>::None;
    loop {
        let now = Instant::now();
        let mut layout = row_spaced::<Context>(
            20.,
            vec![
                conditional(
                    show_alignment == HighlightedCase::HeightConstraints
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        draw(text("Height Constraints", 15., WHITE)).size(Size::new().height(20.)),
                        stack(vec![
                            rect(BLUE),
                            rect(WHITE).size(
                                Size::new()
                                    .height_relative(0.1)
                                    .y_align(YAlign::Top)
                                    .min_height(20.),
                            ),
                            rect(WHITE).size(
                                Size::new()
                                    .height_relative(0.1)
                                    .y_align(YAlign::Bottom)
                                    .min_height(20.),
                            ),
                            rect(WHITE).size(Size::new().height(30.)),
                        ]),
                        // draw(|area, (highlight: HighlightedCase, anim: Animated<f32, Instant>))| {
                        //     if button(area, "Fullscreen", &mut root_ui()) {
                        //         if highlight == HighlightedCase::HeightConstraints {
                        //             anim.transition(1., now);
                        //         } else {
                        //             anim.transition(1., now);
                        //         }
                        //     }
                        // })
                        // .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment == HighlightedCase::WidthConstraints
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        draw(text("Width Constraints", 15., WHITE)).size(Size::new().height(20.)),
                        stack(vec![
                            rect(RED),
                            rect(WHITE).size(
                                Size::new()
                                    .width_relative(0.1)
                                    .x_align(XAlign::Leading)
                                    .min_width(20.),
                            ),
                            rect(WHITE).size(
                                Size::new()
                                    .width_relative(0.1)
                                    .x_align(XAlign::Trailing)
                                    .min_width(20.),
                            ),
                            rect(WHITE).size(Size::new().width(30.)),
                        ]),
                        // draw(|area, show_alignment: &mut Animated<f32, Instant>| {
                        //     if button(area, "Fullscreen", &mut root_ui()) {
                        //         if show_alignment.value == HighlightedCase::WidthConstraints {
                        //             show_alignment.transition(HighlightedCase::None, now);
                        //         } else {
                        //             show_alignment
                        //                 .transition(HighlightedCase::HeightConstraints, now);
                        //         }
                        //     }
                        // })
                        // .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment == HighlightedCase::RelAbsSequence
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        draw(text("Mixed (rel/abs) Sequence Constraints", 15., WHITE))
                            .size(Size::new().height(20.)),
                        stack(vec![
                            rect(BLUE),
                            column_spaced(
                                10.,
                                vec![
                                    rect(WHITE),
                                    rect(WHITE).size(Size::new().height(30.)),
                                    rect(WHITE),
                                ],
                            )
                            .pad(10.),
                        ]),
                        // draw(|area, show_alignment: &mut Animated<f32, Instant>| {
                        //     if button(area, "Fullscreen", &mut root_ui()) {
                        //         if show_alignment.value == HighlightedCase::RelAbsSequence {
                        //             show_alignment.transition(HighlightedCase::None, now);
                        //         } else {
                        //             show_alignment.transition(HighlightedCase::RelAbsSequence, now);
                        //         }
                        //     }
                        // })
                        // .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment == HighlightedCase::AlignmentOffset
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        draw(text("Alignment & Offset", 15., WHITE)).size(Size::new().height(20.)),
                        stack(vec![
                            rect(BLUE),
                            rect(WHITE)
                                .size(Size::new().height(30.).width(30.).x_align(XAlign::Leading)),
                            rect(WHITE)
                                .size(Size::new().height(30.).width(30.).x_align(XAlign::Trailing)),
                            rect(WHITE)
                                .size(Size::new().height(30.).width(30.).y_align(YAlign::Top)),
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
                        draw(|area, ctx: &mut Context| {
                            if button(area, "Fullscreen") {
                                if *ctx.highlight == HighlightedCase::AlignmentOffset {
                                    *ctx.highlight = HighlightedCase::None;
                                } else {
                                    *ctx.highlight = HighlightedCase::AlignmentOffset;
                                }
                                *ctx.anim = Animated::new(0.);
                                ctx.anim.transition(1., Instant::now());
                            }
                            todo!();
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
            ],
        );
        layout.layout(Area {
            x: 0.,
            y: 0.,
            width: screen_width(),
            height: screen_height(),
        });
        // if let Some(ref last) = last_drawables {
        // layout.process_drawables(|drawable| {
        //     if let Some(draw) = drawable.draw.take() {
        //         (draw)(drawable.area, &mut (&mut show_alignment, &mut anim));
        //     }
        // });
        // let interpolated_area = anim
        //     .animate(
        //         |v| {
        //             if v == 0. {
        //                 IArea(*last)
        //             } else {
        //                 IArea(drawable.area)
        //             }
        //         },
        //         now,
        //     )
        //     .0;
        // (drawable.draw)(drawable.area, &mut (&mut show_alignment, &mut anim));
        // (draw)(drawable.area, &mut (&mut show_alignment, &mut anim));

        for mut drawable in layout.drawables() {
            // for i in 0..layout.drawables().len() - 1 {
            if let Some(draw) = drawable.draw.take() {
                thing(
                    drawable.area,
                    &mut Context {
                        highlight: &mut show_alignment,
                        anim: &mut anim,
                    },
                    draw,
                );
                // (draw)(
                //     drawable.area,
                //     &mut Context {
                //         highlight: &mut show_alignment,
                //         anim: &mut anim,
                //     },
                // );
            }
            // }
        }

        // } else {
        // }
        // last_drawables = Some(layout.drawables().iter().map(|d| d.area).collect());

        next_frame().await
    }
}

fn thing<F>(area: Area, ctx: &mut Context, dbf: F)
where
    F: FnOnce(Area, &mut Context),
{
    (dbf)(area, ctx)
}

fn text<T>(string: &str, font_size: f32, color: Color) -> impl FnMut(Area, &mut T) + '_ {
    move |area: Area, _| {
        let dimensions = measure_text(string, None, font_size as u16, 1.0);
        draw_text(
            string,
            area.x + ((area.width - dimensions.width) * 0.5),
            area.y + (area.height * 0.5) + (dimensions.height * 0.5),
            font_size,
            color,
        );
    }
}

fn rect<'a, T>(color: Color) -> Layout<'a, T> {
    draw(move |area: Area, _| {
        draw_rectangle(area.x, area.y, area.width, area.height, color);
    })
}

fn button(area: Area, label: &str) -> bool {
    widgets::Button::new(label)
        .size(vec2(area.width, area.height))
        .position(vec2(area.x, area.y))
        .ui(&mut root_ui())
}
