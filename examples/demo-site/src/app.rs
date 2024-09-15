use backer::{models::*, nodes::*, Layout, Node};
use egui::{
    include_image, text::LayoutJob, Align as EguiAlign, Color32, Image, ImageSource, Label, Pos2,
    Rect, Stroke, Ui,
};

#[derive(Default)]
pub struct TemplateApp {
    zoom_set: bool,
    web: bool,
}

impl TemplateApp {
    pub fn new(web: bool) -> Self {
        Self {
            zoom_set: false,
            web,
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.web && !self.zoom_set {
            self.zoom_set = true;
            ctx.set_zoom_factor(ctx.screen_rect().size().x / 2100.);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout::new(my_layout_fn);
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            layout.draw(available_area, ui);
        });
    }
}

const DEMO_BG: Color32 = Color32::from_rgb(25, 25, 27);
const DEMO_GRAY: Color32 = Color32::from_rgb(50, 50, 50);
const DEMO_DESTRUCTIVE: Color32 = Color32::from_rgb(255, 100, 100);
const DEMO_DESTRUCTIVE_SECONDARY: Color32 = Color32::from_rgb(210, 40, 40);
const DEMO_HINT: Color32 = Color32::from_rgb(35, 35, 38);
const DEMO_FG: Color32 = Color32::from_rgb(250, 250, 255);
const DEMO_FG_SECONDARY: Color32 = Color32::from_rgb(180, 180, 183);
const SCALE: f32 = 1.0;

fn my_layout_fn(ui: &mut Ui) -> Node<Ui> {
    stack(vec![
        rect(Color32::TRANSPARENT, DEMO_BG, 0.),
        row(vec![
            side_bar(ui),
            row_divider(DEMO_GRAY).width(1.),
            column(vec![
                header(ui),
                col_divider(DEMO_GRAY).height(1.),
                main_view(ui),
                space(),
                col_divider(DEMO_GRAY).height(1.),
                footer(ui),
            ])
            .align(Align::TopLeading),
        ]),
    ])
}

fn footer(ui: &mut Ui) -> Node<Ui> {
    row_spaced(
        20. * SCALE,
        vec![
            row_spaced(
                20. * SCALE,
                vec![
                    draw_label_color(ui, "Game", 12., DEMO_FG_SECONDARY),
                    draw_label_color(ui, "Terms & Conditions", 12., DEMO_FG_SECONDARY),
                    draw_label_color(ui, "Privacy Policy", 12., DEMO_FG_SECONDARY),
                ],
            )
            .x_align(XAlign::Leading),
            space(),
            draw_label_color(
                ui,
                "© Backer 2021. All rights reserved",
                12.,
                DEMO_FG_SECONDARY,
            )
            .width(300. * SCALE),
        ],
    )
    .pad(10. * SCALE)
    .height(40. * SCALE)
}

fn main_view(ui: &mut Ui) -> Node<Ui> {
    let profile_blurb = "Your public profile URL can be shared with anyone and allows them to immediately see your bases and activity in Backer.";
    let pic_blurb = "Upload a profile picture of yourself or the character you always wanted to be. Your avatar will be displayed all over the Backer world.";
    let info_blurb = "Tell the world about yourself. Information you add will be visible only in your profile, not for all users.";
    stack(vec![stack(vec![
        rect(DEMO_GRAY, DEMO_HINT, 5.),
        column_spaced(
            10. * SCALE,
            vec![
                row_spaced(
                    10. * SCALE,
                    vec![
                        column_spaced(
                            10. * SCALE,
                            vec![
                                draw_label(ui, "Public profile", 18.),
                                multiline_label(ui, profile_blurb, 10.).height(50. * SCALE),
                            ],
                        )
                        .align(Align::TopLeading)
                        .width_range((80.0 * SCALE)..),
                        column_spaced(
                            10. * SCALE,
                            vec![
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    row_spaced(
                                        10. * SCALE,
                                        vec![
                                            draw_label_color(
                                                ui,
                                                "ejjonny.github.io/backer/username",
                                                12.,
                                                DEMO_FG_SECONDARY,
                                            ),
                                            icon(include_image!("../assets/copy.svg")).aspect(1.),
                                        ],
                                    )
                                    .pad(3. * SCALE),
                                ])
                                .height(25. * SCALE),
                                row_spaced(
                                    10. * SCALE,
                                    vec![
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            row_spaced(
                                                10. * SCALE,
                                                vec![
                                                    icon(include_image!("../assets/share.svg"))
                                                        .aspect(1.),
                                                    draw_label_color(
                                                        ui,
                                                        "Share",
                                                        12.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            )
                                            .pad(3. * SCALE),
                                        ])
                                        .height(25. * SCALE),
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            row_spaced(
                                                10. * SCALE,
                                                vec![
                                                    icon(include_image!("../assets/map-pin.svg"))
                                                        .aspect(1.),
                                                    draw_label_color(
                                                        ui,
                                                        "View location",
                                                        12.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            )
                                            .pad(3. * SCALE),
                                        ])
                                        .height(25. * SCALE),
                                    ],
                                ),
                            ],
                        ),
                    ],
                )
                .y_align(YAlign::Top),
                col_divider(DEMO_GRAY).height(1.),
                row_spaced(
                    10. * SCALE,
                    vec![
                        column_spaced(
                            10. * SCALE,
                            vec![
                                draw_label(ui, "Edit PFP", 18.),
                                multiline_label(ui, pic_blurb, 10.).height(50. * SCALE),
                            ],
                        )
                        .align(Align::TopLeading)
                        .width_range((80.0 * SCALE)..),
                        row_spaced(
                            10. * SCALE,
                            vec![
                                rect(DEMO_FG, DEMO_BG, 100.)
                                    .height(30. * SCALE)
                                    .width(30. * SCALE),
                                column_spaced(
                                    5. * SCALE,
                                    vec![
                                        draw_label(ui, "@UserName", 12.),
                                        draw_label_color(
                                            ui,
                                            "Living, laughing, loving",
                                            10.,
                                            DEMO_FG_SECONDARY,
                                        ),
                                    ],
                                )
                                .x_align(XAlign::Leading),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    draw_label_color(ui, "Upload", 12., DEMO_FG_SECONDARY)
                                        .pad(5. * SCALE),
                                ])
                                .height(25. * SCALE),
                                stack(vec![
                                    rect(DEMO_DESTRUCTIVE_SECONDARY, DEMO_BG, 5.),
                                    draw_label_color(ui, "Remove", 12., DEMO_DESTRUCTIVE)
                                        .pad(5. * SCALE),
                                ])
                                .height(25. * SCALE),
                            ],
                        ),
                    ],
                ),
                col_divider(DEMO_GRAY).height(1.),
                row_spaced(
                    10. * SCALE,
                    vec![
                        column_spaced(
                            10. * SCALE,
                            vec![
                                draw_label(ui, "Edit personal information", 18.),
                                multiline_label(ui, info_blurb, 10.).height(50. * SCALE),
                            ],
                        )
                        .align(Align::TopLeading)
                        .width_range((80.0 * SCALE)..),
                        column_spaced(
                            5. * SCALE,
                            vec![
                                draw_label_color(ui, "Edit username", 12., DEMO_FG_SECONDARY),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    draw_label_color(ui, "@UserName", 12., DEMO_FG)
                                        .x_align(XAlign::Leading)
                                        .pad(5. * SCALE),
                                ])
                                .height(25. * SCALE),
                                draw_label_color(ui, "Bio", 12., DEMO_FG_SECONDARY),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    draw_label_color(ui, "Living, laughing, loving", 12., DEMO_FG)
                                        .align(Align::TopLeading)
                                        .pad(5. * SCALE),
                                ])
                                .height(50. * SCALE),
                            ],
                        )
                        .x_align(XAlign::Leading),
                    ],
                )
                .y_align(YAlign::Top),
            ],
        )
        .align(Align::TopLeading)
        .pad_y(40. * SCALE)
        .pad_x(30. * SCALE),
        rect_stroke(DEMO_GRAY),
    ])
    .pad(20. * SCALE)])
}

fn side_bar(ui: &mut Ui) -> Node<Ui> {
    column_spaced(
        15. * SCALE,
        vec![
            draw_label(ui, "BACKER", 22.).height(35. * SCALE),
            col_divider(DEMO_GRAY).pad_x(-30. * SCALE).height(1.),
            draw_label(ui, "Home", 10.),
            draw_label(ui, "Explore", 10.),
            draw_label(ui, "Marketplace", 10.),
            draw_label(ui, "My Account", 10.),
            col_divider(DEMO_GRAY).pad_trailing(-20. * SCALE).height(1.),
            draw_label(ui, "Activity", 10.),
            draw_label(ui, "News", 10.),
            draw_label(ui, "Docs", 10.),
            col_divider(DEMO_GRAY).pad_trailing(-20. * SCALE).height(1.),
            draw_label(ui, "Twitter", 10.),
            draw_label(ui, "Telegram", 10.),
            draw_label(ui, "Medium", 10.),
            space(),
        ],
    )
    .align(Align::TopLeading)
    .pad(30. * SCALE)
    .width(150. * SCALE)
}

fn header(ui: &mut Ui) -> Node<Ui> {
    row_spaced(
        10. * SCALE,
        vec![
            draw_label(ui, "My Account", 18.)
                .y_align(YAlign::Bottom)
                .width(110. * SCALE),
            space(),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                draw_label(ui, "$115,000", 12.),
            ])
            .width(80. * SCALE),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                row(vec![draw_label(ui, "Operational", 12.)]),
            ])
            .width(90. * SCALE),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/bell.svg"))
                    .aspect(1.)
                    .pad_y(8.5 * SCALE)
                    .aspect(1.),
            ])
            .aspect(1.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/user.svg"))
                    .aspect(1.)
                    .pad_y(8.5 * SCALE)
                    .aspect(1.),
            ])
            .aspect(1.),
        ],
    )
    .pad_top(35. * SCALE)
    .pad_bottom(15. * SCALE)
    .pad_x(30. * SCALE)
    .height(80. * SCALE)
}

fn icon(image: impl Into<ImageSource<'static>> + 'static) -> Node<Ui> {
    let image = Image::new(image).tint(Color32::WHITE);
    draw(move |area, ui: &mut Ui| {
        ui.put(rect_from(area), image.clone());
    })
}

fn multiline_label<S: AsRef<str> + 'static>(_ui: &'_ mut Ui, text: S, size: f32) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        let layout_job = LayoutJob::single_section(
            text.as_ref().to_string(),
            egui::TextFormat {
                font_id: egui::FontId::new(size * SCALE, egui::FontFamily::Proportional),
                extra_letter_spacing: 0.,
                line_height: Some(14. * SCALE),
                color: Color32::WHITE,
                background: Color32::TRANSPARENT,
                italics: false,
                underline: Stroke::NONE,
                strikethrough: Stroke::NONE,
                valign: EguiAlign::Min,
            },
        );
        let rect = rect_from(area);
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical(|ui| {
                ui.add(Label::new(layout_job));
            })
        });
    })
}

fn draw_label<S: AsRef<str> + 'static>(ui: &'_ mut Ui, text: S, size: f32) -> Node<Ui> {
    draw_label_color(ui, text, size, DEMO_FG)
}

fn draw_label_color<S: AsRef<str> + 'static>(
    ui: &'_ mut Ui,
    text: S,
    size: f32,
    color: Color32,
) -> Node<Ui> {
    let job = LayoutJob::simple(
        text.as_ref().to_string(),
        egui::FontId::new(size * SCALE, egui::FontFamily::Proportional),
        color,
        200.,
    );
    let label = egui::Label::new(job);
    let galley = label.layout_in_ui(ui).1.rect;
    let text_area = area_from(galley);
    draw(move |area, ui: &mut Ui| {
        ui.put(
            rect_from(area),
            egui::Label::new(LayoutJob::simple(
                text.as_ref().to_string(),
                egui::FontId::new(size * SCALE, egui::FontFamily::Proportional),
                color,
                200.,
            )),
        );
    })
    .width(text_area.width)
    .height(text_area.height)
}

fn col_divider(color: Color32) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        ui.painter().line_segment(
            [
                Pos2::new(area.x, area.y + (area.height * 0.5)),
                Pos2::new(area.x + area.width, area.y + (area.height * 0.5)),
            ],
            Stroke::new(1., color),
        );
    })
}
fn row_divider(color: Color32) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        ui.painter().line_segment(
            [
                Pos2::new(area.x + (area.width * 0.5), area.y),
                Pos2::new(area.x + (area.width * 0.5), area.y + area.height),
            ],
            Stroke::new(1., color),
        );
    })
}

fn rect(stroke: Color32, fill: Color32, rounding: f32) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        ui.painter()
            .rect_stroke(rect_from(area), rounding * SCALE, Stroke::new(1., stroke));
        ui.painter().rect_filled(rect_from(area), rounding, fill);
    })
}

fn rect_stroke(color: Color32) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        ui.painter()
            .rect_stroke(rect_from(area), 5. * SCALE, Stroke::new(1., color));
    })
}

fn area_from(rect: Rect) -> Area {
    Area {
        x: rect.min.x,
        y: rect.min.y,
        width: rect.max.x - rect.min.x,
        height: rect.max.y - rect.min.y,
    }
}

fn rect_from(area: Area) -> Rect {
    Rect {
        min: Pos2::new(area.x, area.y),
        max: Pos2::new(area.x + area.width, area.y + area.height),
    }
}
