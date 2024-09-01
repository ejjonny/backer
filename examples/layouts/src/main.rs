use backer::models::*;
use backer::nodes::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;
use macroquad::ui::Ui;

#[derive(Debug, Clone, PartialEq, Eq)]
enum HighlightedCase {
    HeightConstraints,
    WidthConstraints,
    RelAbsSequence,
    AlignmentOffset,
}

#[macroquad::main("Demo")]
async fn main() {
    let mut show_alignment: Option<HighlightedCase> = None;
    loop {
        row_spaced(
            20.,
            vec![
                conditional(
                    show_alignment
                        .as_ref()
                        .is_some_and(|a| *a == HighlightedCase::HeightConstraints)
                        || show_alignment.is_none(),
                    column(vec![
                        draw(text("Height Constraints", 15., WHITE)).size(Size::new().height(20.)),
                        stack(vec![
                            draw(rect(BLUE)),
                            draw(rect(WHITE)).size(
                                Size::new()
                                    .height_relative(0.1)
                                    .y_align(YAlign::Top)
                                    .min_height(20.),
                            ),
                            draw(rect(WHITE)).size(
                                Size::new()
                                    .height_relative(0.1)
                                    .y_align(YAlign::Bottom)
                                    .min_height(20.),
                            ),
                            draw(rect(WHITE)).size(Size::new().height(30.)),
                        ]),
                        draw(|area, show_alignment| {
                            if button(area, "Fullscreen", &mut root_ui()) {
                                if *show_alignment == Some(HighlightedCase::HeightConstraints) {
                                    *show_alignment = None;
                                } else {
                                    *show_alignment = Some(HighlightedCase::HeightConstraints);
                                }
                            }
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment
                        .as_ref()
                        .is_some_and(|a| *a == HighlightedCase::WidthConstraints)
                        || show_alignment.is_none(),
                    column(vec![
                        draw(text("Width Constraints", 15., WHITE)).size(Size::new().height(20.)),
                        stack(vec![
                            draw(rect(RED)),
                            draw(rect(WHITE)).size(
                                Size::new()
                                    .width_relative(0.1)
                                    .x_align(XAlign::Leading)
                                    .min_width(20.),
                            ),
                            draw(rect(WHITE)).size(
                                Size::new()
                                    .width_relative(0.1)
                                    .x_align(XAlign::Trailing)
                                    .min_width(20.),
                            ),
                            draw(rect(WHITE)).size(Size::new().width(30.)),
                        ]),
                        draw(|area, show_alignment| {
                            if button(area, "Fullscreen", &mut root_ui()) {
                                if *show_alignment == Some(HighlightedCase::WidthConstraints) {
                                    *show_alignment = None;
                                } else {
                                    *show_alignment = Some(HighlightedCase::WidthConstraints);
                                }
                            }
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment
                        .as_ref()
                        .is_some_and(|a| *a == HighlightedCase::RelAbsSequence)
                        || show_alignment.is_none(),
                    column(vec![
                        draw(text("Mixed (rel/abs) Sequence Constraints", 15., WHITE))
                            .size(Size::new().height(20.)),
                        stack(vec![
                            draw(rect(BLUE)),
                            column_spaced(
                                10.,
                                vec![
                                    draw(rect(WHITE)),
                                    draw(rect(WHITE)).size(Size::new().height(30.)),
                                    draw(rect(WHITE)),
                                ],
                            )
                            .pad(10.),
                        ]),
                        draw(|area, show_alignment| {
                            if button(area, "Fullscreen", &mut root_ui()) {
                                if *show_alignment == Some(HighlightedCase::RelAbsSequence) {
                                    *show_alignment = None;
                                } else {
                                    *show_alignment = Some(HighlightedCase::RelAbsSequence);
                                }
                            }
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
                conditional(
                    show_alignment
                        .as_ref()
                        .is_some_and(|a| *a == HighlightedCase::AlignmentOffset)
                        || show_alignment.is_none(),
                    column(vec![
                        draw(text("Alignment & Offset", 15., WHITE)).size(Size::new().height(20.)),
                        stack(vec![
                            draw(rect(BLUE)),
                            draw(rect(WHITE))
                                .size(Size::new().height(30.).width(30.).x_align(XAlign::Leading)),
                            draw(rect(WHITE))
                                .size(Size::new().height(30.).width(30.).x_align(XAlign::Trailing)),
                            draw(rect(WHITE))
                                .size(Size::new().height(30.).width(30.).y_align(YAlign::Top)),
                            draw(rect(WHITE))
                                .size(Size::new().height(30.).width(30.).y_align(YAlign::Bottom)),
                            draw(rect(WHITE))
                                .size(Size::new().height(30.).width(30.).align(Align::TopLeading)),
                            draw(rect(WHITE)).size(
                                Size::new()
                                    .height(30.)
                                    .width(30.)
                                    .align(Align::BottomLeading),
                            ),
                            draw(rect(WHITE)).size(
                                Size::new()
                                    .height(30.)
                                    .width(30.)
                                    .align(Align::BottomTrailing),
                            ),
                            draw(rect(WHITE))
                                .size(Size::new().height(30.).width(30.).align(Align::TopTrailing)),
                            draw(rect(WHITE))
                                .size(
                                    Size::new()
                                        .height(30.)
                                        .width(30.)
                                        .align(Align::CenterCenter),
                                )
                                .offset(10., 10.),
                            draw(rect(WHITE))
                                .size(
                                    Size::new()
                                        .height(30.)
                                        .width(30.)
                                        .align(Align::CenterCenter),
                                )
                                .offset(-10., -10.),
                        ]),
                        draw(|area, show_alignment| {
                            if button(area, "Fullscreen", &mut root_ui()) {
                                if *show_alignment == Some(HighlightedCase::AlignmentOffset) {
                                    *show_alignment = None;
                                } else {
                                    *show_alignment = Some(HighlightedCase::AlignmentOffset);
                                }
                            }
                        })
                        .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                    ]),
                ),
            ],
        )
        .layout(
            Area {
                x: 0.,
                y: 0.,
                width: screen_width(),
                height: screen_height(),
            },
            &mut show_alignment,
        );

        next_frame().await
    }
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

fn rect<T>(color: Color) -> impl FnMut(Area, &mut T) {
    move |area: Area, _| {
        draw_rectangle(area.x, area.y, area.width, area.height, color);
    }
}

fn button(area: Area, label: &str, ui: &mut Ui) -> bool {
    widgets::Button::new(label)
        .size(vec2(area.width, area.height))
        .position(vec2(area.x, area.y))
        .ui(ui)
}
