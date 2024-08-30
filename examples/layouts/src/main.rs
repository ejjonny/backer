use backer::models::*;
use backer::modifiers::*;
use backer::nodes::*;
use macroquad::prelude::*;

#[macroquad::main("Demo")]
async fn main() {
    loop {
        let layout = row_spaced(
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
                    draw(MyDrawable::Text {
                        string: "Alignment & Offset".to_string(),
                        font_size: 15.,
                        color: WHITE,
                    })
                    .size(Size::new().height(20.)),
                    stack(vec![
                        draw(MyDrawable::Rect(RED)),
                        draw(MyDrawable::Rect(WHITE))
                            .size(Size::new().height(30.).width(30.).x_align(XAlign::Leading)),
                        draw(MyDrawable::Rect(WHITE))
                            .size(Size::new().height(30.).width(30.).x_align(XAlign::Trailing)),
                        draw(MyDrawable::Rect(WHITE))
                            .size(Size::new().height(30.).width(30.).y_align(YAlign::Top)),
                        draw(MyDrawable::Rect(WHITE))
                            .size(Size::new().height(30.).width(30.).y_align(YAlign::Bottom)),
                        draw(MyDrawable::Rect(WHITE))
                            .size(Size::new().height(30.).width(30.).align(Align::TopLeading)),
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
                        draw(MyDrawable::Rect(WHITE))
                            .size(Size::new().height(30.).width(30.).align(Align::TopTrailing)),
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
            ],
        );
        let mut repeated = column((0..4).map(|_| layout.clone()).collect());
        repeated.layout(Area {
            x: 0.,
            y: 0.,
            width: screen_width(),
            height: screen_height(),
        });

        repeated
            .drawables()
            .iter()
            .for_each(|drawable| drawable.element.draw(drawable.area));

        next_frame().await
    }
}

#[derive(Debug, Clone)]
enum MyDrawable {
    Rect(Color),
    Text {
        string: String,
        font_size: f32,
        color: Color,
    },
}

impl MyDrawable {
    fn draw(&self, area: Area) {
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
        }
    }
}
