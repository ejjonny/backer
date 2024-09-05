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

struct Context {
    highlight: HighlightedCase,
    anim: Animated<HighlightedCase, Instant>,
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

struct IVec<T>(Vec<T>)
where
    T: Interpolable;

impl<T> Interpolable for IVec<T>
where
    T: Interpolable,
{
    fn interpolated(&self, other: Self, ratio: f32) -> Self {
        IVec(
            std::iter::zip(self.0.iter(), other.0)
                .map(|(a, b)| a.interpolated(b, ratio))
                .collect(),
        )
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

#[macroquad::main("Demo")]
async fn main() {
    let mut highlight: HighlightedCase = HighlightedCase::None;
    let mut anim = Animated::<HighlightedCase, Instant>::new(highlight)
        .duration(300.)
        .easing(lilt::Easing::EaseInOut);
    loop {
        let mut layout = layout_for_highlight(highlight);
        layout.layout(Area {
            x: 0.,
            y: 0.,
            width: screen_width(),
            height: screen_height(),
        });
        {
            let now = Instant::now();
            let areas: Vec<Area> = anim
                .clone()
                .animate(
                    |m_highlight| {
                        IVec((|| {
                            let mut lt = layout_for_highlight(m_highlight);
                            lt.layout(Area {
                                x: 0.,
                                y: 0.,
                                width: screen_width(),
                                height: screen_height(),
                            });
                            return lt
                                .drawables()
                                .into_iter()
                                .map(|d| IArea(d.area))
                                .collect::<Vec<IArea>>();
                        })())
                    },
                    now,
                )
                .0
                .into_iter()
                .map(|i| i.0)
                .collect();
            let mut ctx = Context { highlight, anim };
            layout
                .drawables()
                .into_iter()
                .enumerate()
                .for_each(|(i, d)| d.draw(areas[i], &mut ctx));
            anim = ctx.anim;
            highlight = ctx.highlight;
        }
        next_frame().await
    }
}

fn layout_for_highlight(highlight: HighlightedCase) -> Layout<Context> {
    let button_size = 50.;
    row_spaced(
        20.,
        vec![
            conditional(
                highlight == HighlightedCase::HeightConstraints
                    || highlight == HighlightedCase::None,
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
                        let now = Instant::now();
                        if ctx.highlight == HighlightedCase::HeightConstraints {
                            ctx.highlight = HighlightedCase::None;
                            ctx.anim.transition(HighlightedCase::None, now);
                        } else {
                            ctx.highlight = HighlightedCase::HeightConstraints;
                            ctx.anim.transition(HighlightedCase::HeightConstraints, now);
                        }
                    })
                    .size(Size::new().height(button_size).y_align(YAlign::Bottom)),
                ]),
            ),
            conditional(
                highlight == HighlightedCase::WidthConstraints
                    || highlight == HighlightedCase::None,
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
                        let now = Instant::now();
                        if ctx.highlight == HighlightedCase::WidthConstraints {
                            ctx.highlight = HighlightedCase::None;
                            ctx.anim.transition(HighlightedCase::None, now);
                        } else {
                            ctx.highlight = HighlightedCase::WidthConstraints;
                            ctx.anim.transition(HighlightedCase::WidthConstraints, now);
                        }
                    })
                    .size(Size::new().height(button_size).y_align(YAlign::Bottom)),
                ]),
            ),
            conditional(
                highlight == HighlightedCase::RelAbsSequence || highlight == HighlightedCase::None,
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
                        let now = Instant::now();
                        if ctx.highlight == HighlightedCase::RelAbsSequence {
                            ctx.highlight = HighlightedCase::None;
                            ctx.anim.transition(HighlightedCase::None, now);
                        } else {
                            ctx.highlight = HighlightedCase::RelAbsSequence;
                            ctx.anim.transition(HighlightedCase::RelAbsSequence, now);
                        }
                    })
                    .size(Size::new().height(button_size).y_align(YAlign::Bottom)),
                ]),
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
                    button("Fullscreen", |ctx: &mut Context| {
                        let now = Instant::now();
                        if ctx.highlight == HighlightedCase::AlignmentOffset {
                            ctx.highlight = HighlightedCase::None;
                            ctx.anim.transition(HighlightedCase::None, now);
                        } else {
                            ctx.highlight = HighlightedCase::AlignmentOffset;
                            ctx.anim.transition(HighlightedCase::AlignmentOffset, now);
                        }
                    })
                    .size(Size::new().height(button_size).y_align(YAlign::Bottom)),
                ]),
            ),
        ],
    )
}

fn text<U>(string: &'static str, font_size: f32, color: Color) -> Layout<U> {
    draw(move |area: Area, _| {
        let dimensions = measure_text(string, None, font_size as u16, 1.0);
        draw_text(
            string,
            area.x + ((area.width - dimensions.width) * 0.5),
            area.y + (area.height * 0.5) + (dimensions.height * 0.5),
            font_size,
            color,
        );
    })
}

fn rect<U>(color: Color) -> Layout<U> {
    draw(move |area: Area, _| {
        draw_rectangle(area.x, area.y, area.width, area.height, color);
    })
}

fn button<U, Action>(label: &'static str, action: Action) -> Layout<U>
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
