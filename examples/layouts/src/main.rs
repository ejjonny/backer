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
        let mut layout = row_spaced(
            20.,
            vec![
                conditional(
                    show_alignment == HighlightedCase::HeightConstraints
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        text("Height Constraints", 15., WHITE).size(Size::new().height(20.)),
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
                        button("Fullscreen", |ctx: &mut Context| {
                            if *ctx.highlight == HighlightedCase::HeightConstraints {
                                *ctx.highlight = HighlightedCase::None;
                            } else {
                                *ctx.highlight = HighlightedCase::HeightConstraints;
                            }
                            *ctx.anim = Animated::new(0.);
                            ctx.anim.transition(1., Instant::now());
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment == HighlightedCase::WidthConstraints
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        text("Width Constraints", 15., WHITE).size(Size::new().height(20.)),
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
                        button("Fullscreen", |ctx: &mut Context| {
                            if *ctx.highlight == HighlightedCase::WidthConstraints {
                                *ctx.highlight = HighlightedCase::None;
                            } else {
                                *ctx.highlight = HighlightedCase::WidthConstraints;
                            }
                            *ctx.anim = Animated::new(0.);
                            ctx.anim.transition(1., Instant::now());
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment == HighlightedCase::RelAbsSequence
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        text("Mixed (rel/abs) Sequence Constraints", 15., WHITE)
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
                        button("Fullscreen", |ctx: &mut Context| {
                            if *ctx.highlight == HighlightedCase::RelAbsSequence {
                                *ctx.highlight = HighlightedCase::None;
                            } else {
                                *ctx.highlight = HighlightedCase::RelAbsSequence;
                            }
                            *ctx.anim = Animated::new(0.);
                            ctx.anim.transition(1., Instant::now());
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment == HighlightedCase::AlignmentOffset
                        || show_alignment == HighlightedCase::None,
                    column(vec![
                        text("Alignment & Offset", 15., WHITE).size(Size::new().height(20.)),
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
                        button("Fullscreen", |ctx: &mut Context| {
                            if *ctx.highlight == HighlightedCase::AlignmentOffset {
                                *ctx.highlight = HighlightedCase::None;
                            } else {
                                *ctx.highlight = HighlightedCase::AlignmentOffset;
                            }
                            *ctx.anim = Animated::new(0.);
                            ctx.anim.transition(1., Instant::now());
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
        let mut ctx = Context {
            highlight: &mut show_alignment,
            anim: &mut anim,
        };
        for drawable in layout.drawables() {
            drawable.draw(drawable.area, &mut ctx);
        }
        last_drawables = Some(layout.drawables().iter().map(|d| d.area).collect());
        next_frame().await
    }
}

type CustomDrawable<U> = Layout<Box<dyn Fn(Area, &mut U)>, U>;

fn text<U>(string: &'static str, font_size: f32, color: Color) -> CustomDrawable<U> {
    draw(Box::new(move |area: Area, _| {
        let dimensions = measure_text(string, None, font_size as u16, 1.0);
        draw_text(
            string,
            area.x + ((area.width - dimensions.width) * 0.5),
            area.y + (area.height * 0.5) + (dimensions.height * 0.5),
            font_size,
            color,
        );
    }))
}

fn rect<U>(color: Color) -> CustomDrawable<U> {
    draw(Box::new(move |area: Area, _| {
        draw_rectangle(area.x, area.y, area.width, area.height, color);
    }))
}

fn button<U, Action>(label: &'static str, action: Action) -> CustomDrawable<U>
where
    Action: Fn(&mut U) + 'static,
{
    draw(Box::new(move |area: Area, ctx: &mut U| {
        if widgets::Button::new(label)
            .size(vec2(area.width, area.height))
            .position(vec2(area.x, area.y))
            .ui(&mut root_ui())
        {
            action(ctx)
        }
    }))
}
