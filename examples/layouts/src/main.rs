use backer::models::*;
use backer::nodes::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;
use macroquad::ui::Ui;

#[macroquad::main("Demo")]
async fn main() {
    let mut show_alignment = false;
    loop {
        let show_alignment_value = show_alignment;
        let mut layout = row_spaced(
            20.,
            vec![
                column(vec![
                    draw(MyDrawable::Text {
                        string: "Height Constraints".to_string(),
                        font_size: 15.,
                        color: WHITE,
                    })
                    .size(Size::new().height(20.)),
                    stack(vec![
                        draw(MyDrawable::Rect(BLUE)),
                        draw(MyDrawable::Rect(WHITE)).size(
                            Size::new()
                                .height_relative(0.1)
                                .y_align(YAlign::Top)
                                .min_height(20.),
                        ),
                        draw(MyDrawable::Rect(WHITE)).size(
                            Size::new()
                                .height_relative(0.1)
                                .y_align(YAlign::Bottom)
                                .min_height(20.),
                        ),
                        draw(MyDrawable::Rect(WHITE)).size(Size::new().height(30.)),
                    ]),
                ]),
                column(vec![
                    draw(MyDrawable::Text {
                        string: "Width Constraints".to_string(),
                        font_size: 15.,
                        color: WHITE,
                    })
                    .size(Size::new().height(20.)),
                    stack(vec![
                        draw(MyDrawable::Rect(RED)),
                        draw(MyDrawable::Rect(WHITE)).size(
                            Size::new()
                                .width_relative(0.1)
                                .x_align(XAlign::Leading)
                                .min_width(20.),
                        ),
                        draw(MyDrawable::Rect(WHITE)).size(
                            Size::new()
                                .width_relative(0.1)
                                .x_align(XAlign::Trailing)
                                .min_width(20.),
                        ),
                        draw(MyDrawable::Rect(WHITE)).size(Size::new().width(30.)),
                    ]),
                ]),
                column(vec![
                    draw(MyDrawable::Text {
                        string: "Mixed (rel/abs) Sequence Constraints".to_string(),
                        font_size: 15.,
                        color: WHITE,
                    })
                    .size(Size::new().height(20.)),
                    stack(vec![
                        draw(MyDrawable::Rect(BLUE)),
                        column_spaced(
                            10.,
                            vec![
                                draw(MyDrawable::Rect(WHITE)),
                                draw(MyDrawable::Rect(WHITE)).size(Size::new().height(30.)),
                                draw(MyDrawable::Rect(WHITE)),
                            ],
                        )
                        .pad(10.),
                    ]),
                ]),
                column(vec![
                    conditional(
                        show_alignment_value,
                        column(vec![
                            draw(MyDrawable::Text {
                                string: "Alignment & Offset".to_string(),
                                font_size: 15.,
                                color: WHITE,
                            })
                            .size(Size::new().height(20.)),
                            stack(vec![
                                draw(MyDrawable::Rect(RED)),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new().height(30.).width(30.).x_align(XAlign::Leading),
                                ),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new().height(30.).width(30.).x_align(XAlign::Trailing),
                                ),
                                draw(MyDrawable::Rect(WHITE))
                                    .size(Size::new().height(30.).width(30.).y_align(YAlign::Top)),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new().height(30.).width(30.).y_align(YAlign::Bottom),
                                ),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new().height(30.).width(30.).align(Align::TopLeading),
                                ),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new()
                                        .height(30.)
                                        .width(30.)
                                        .align(Align::BottomLeading),
                                ),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new()
                                        .height(30.)
                                        .width(30.)
                                        .align(Align::BottomTrailing),
                                ),
                                draw(MyDrawable::Rect(WHITE)).size(
                                    Size::new().height(30.).width(30.).align(Align::TopTrailing),
                                ),
                                draw(MyDrawable::Rect(WHITE))
                                    .size(
                                        Size::new()
                                            .height(30.)
                                            .width(30.)
                                            .align(Align::CenterCenter),
                                    )
                                    .offset(10., 10.),
                                draw(MyDrawable::Rect(WHITE))
                                    .size(
                                        Size::new()
                                            .height(30.)
                                            .width(30.)
                                            .align(Align::CenterCenter),
                                    )
                                    .offset(-10., -10.),
                            ]),
                        ]),
                    ),
                    draw(MyDrawable::Button {
                        label: "Show".to_string(),
                        action: &mut show_alignment,
                    })
                    .size(Size::new().height(20.).y_align(YAlign::Bottom)),
                ]),
            ],
        );

        layout.layout(Area {
            x: 0.,
            y: 0.,
            width: screen_width(),
            height: screen_height(),
        });

        layout
            .drawables()
            .iter_mut()
            .for_each(|drawable| drawable.element.draw(drawable.area, &mut root_ui()));

        next_frame().await
    }
}

enum MyDrawable<'a> {
    Rect(Color),
    Text {
        string: String,
        font_size: f32,
        color: Color,
    },
    Button {
        label: String,
        action: &'a mut bool,
    },
}

impl<'a> MyDrawable<'a> {
    fn draw(&mut self, area: Area, ui: &mut Ui) {
        match self {
            MyDrawable::Rect(color) => {
                draw_rectangle(area.x, area.y, area.width, area.height, *color);
            }
            MyDrawable::Text {
                string,
                font_size,
                color,
            } => {
                let dimensions = measure_text(string.as_str(), None, *font_size as u16, 1.0);
                draw_text(
                    string.as_str(),
                    area.x + ((area.width - dimensions.width) * 0.5),
                    area.y + (area.height * 0.5) + (dimensions.height * 0.5),
                    *font_size,
                    *color,
                );
            }
            MyDrawable::Button { label, action } => {
                if widgets::Button::new(label.as_str())
                    .size(vec2(area.width, area.height))
                    .position(vec2(area.x, area.y))
                    .ui(ui)
                {
                    **action = !**action
                };
            }
        }
    }
}
