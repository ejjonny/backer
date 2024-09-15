use backer::{models::*, nodes::*, Layout, Node};
use egui::{
    include_image, text::LayoutJob, Align as EguiAlign, Button, Color32, Image, ImageSource, Label,
    Pos2, Rect, Stroke, Ui,
};

#[derive(Default)]
pub struct TemplateApp {
    zoom_set: bool,
    web: bool,
    sidebar: bool,
}

impl TemplateApp {
    pub fn new(web: bool) -> Self {
        Self {
            zoom_set: false,
            web,
            sidebar: false,
        }
    }
}

struct State<'a> {
    ui: &'a mut Ui,
    sidebar: &'a mut bool,
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.web && !self.zoom_set {
            self.zoom_set = true;
            ctx.set_zoom_factor(ctx.screen_rect().size().x.min(1500.) / 1500.);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout::new(my_layout_fn);
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            let mut state = State {
                ui: &mut *ui,
                sidebar: &mut self.sidebar,
            };
            layout.draw(available_area, &mut state);
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

fn my_layout_fn<'a>(state: &mut State<'_>) -> Node<State<'a>> {
    stack(vec![
        rect(Color32::TRANSPARENT, DEMO_BG, 0.),
        row(vec![
            row_divider(DEMO_GRAY).width(1.),
            column(vec![
                header(state),
                col_divider(DEMO_GRAY).height(1.),
                main_view(state),
                space(),
                col_divider(DEMO_GRAY).height(1.),
                footer(state),
            ]),
        ])
        .y_align(YAlign::Top),
        if *state.sidebar {
            side_bar(state)
        } else {
            empty()
        },
    ])
}

fn footer<'a>(state: &mut State<'_>) -> Node<State<'a>> {
    row_spaced(
        10.,
        vec![
            row_spaced(
                20.,
                vec![
                    draw_label_color(state.ui, "Game", 9., DEMO_FG_SECONDARY),
                    draw_label_color(state.ui, "Terms & Conditions", 9., DEMO_FG_SECONDARY),
                    draw_label_color(state.ui, "Privacy Policy", 9., DEMO_FG_SECONDARY),
                ],
            )
            .x_align(XAlign::Leading),
            space(),
            draw_label_color(
                state.ui,
                "© Backer 2021. All rights reserved",
                9.,
                DEMO_FG_SECONDARY,
            )
            .width_range((100.)..),
        ],
    )
    .pad(10.)
    .height(40.)
}

fn main_view<'a>(state: &mut State<'_>) -> Node<State<'a>> {
    let profile_blurb = "Your public profile URL can be shared with anyone and allows them to immediately see your bases and activity in Backer.";
    let pic_blurb = "Upload a profile picture of yourself or the character you always wanted to be. Your avatar will be displayed all over the Backer world.";
    let info_blurb = "Tell the world about yourself. Information you add will be visible only in your profile, not for all users.";
    stack(vec![stack(vec![
        rect(DEMO_GRAY, DEMO_HINT, 5.),
        column_spaced(
            10.,
            vec![
                row_spaced(
                    10.,
                    vec![
                        column_spaced(
                            10.,
                            vec![
                                draw_label(state.ui, "Public profile", 18.),
                                multiline_label(state.ui, profile_blurb, 10.).height(80.),
                            ],
                        )
                        .align(Align::TopLeading)
                        .width_range((80.0)..),
                        column_spaced(
                            10.,
                            vec![
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    row_spaced(
                                        10.,
                                        vec![
                                            draw_label_color(
                                                state.ui,
                                                "ejjonny.io/backer/username",
                                                12.,
                                                DEMO_FG_SECONDARY,
                                            ),
                                            icon(include_image!("../assets/copy.svg")).aspect(1.),
                                        ],
                                    )
                                    .pad(3.),
                                ])
                                .height(25.),
                                row_spaced(
                                    10.,
                                    vec![
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            row_spaced(
                                                10.,
                                                vec![
                                                    icon(include_image!("../assets/share.svg"))
                                                        .aspect(1.),
                                                    draw_label_color(
                                                        state.ui,
                                                        "Share",
                                                        12.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            )
                                            .pad(3.),
                                        ])
                                        .height(25.),
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            row_spaced(
                                                10.,
                                                vec![
                                                    icon(include_image!("../assets/map-pin.svg"))
                                                        .aspect(1.),
                                                    draw_label_color(
                                                        state.ui,
                                                        "View location",
                                                        12.,
                                                        DEMO_FG_SECONDARY,
                                                    ),
                                                ],
                                            )
                                            .pad(3.),
                                        ])
                                        .height(25.),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
                col_divider(DEMO_GRAY).height(1.),
                row_spaced(
                    10.,
                    vec![
                        column_spaced(
                            10.,
                            vec![
                                draw_label(state.ui, "Edit PFP", 18.),
                                multiline_label(state.ui, pic_blurb, 10.).height(80.),
                            ],
                        )
                        .align(Align::TopLeading)
                        .width_range((80.0)..),
                        column_spaced(
                            10.,
                            vec![
                                row_spaced(
                                    10.,
                                    vec![
                                        rect(DEMO_FG, DEMO_BG, 100.).height(30.).width(30.),
                                        column_spaced(
                                            5.,
                                            vec![
                                                draw_label(state.ui, "@UserName", 12.),
                                                draw_label_color(
                                                    state.ui,
                                                    "Living, laughing, loving",
                                                    10.,
                                                    DEMO_FG_SECONDARY,
                                                ),
                                            ],
                                        )
                                        .x_align(XAlign::Leading),
                                    ],
                                ),
                                row_spaced(
                                    10.,
                                    vec![
                                        stack(vec![
                                            rect(DEMO_FG, DEMO_BG, 5.),
                                            draw_label_color(
                                                state.ui,
                                                "Upload",
                                                12.,
                                                DEMO_FG_SECONDARY,
                                            )
                                            .pad(5.),
                                        ])
                                        .height(25.),
                                        stack(vec![
                                            rect(DEMO_DESTRUCTIVE_SECONDARY, DEMO_BG, 5.),
                                            draw_label_color(
                                                state.ui,
                                                "Remove",
                                                12.,
                                                DEMO_DESTRUCTIVE,
                                            )
                                            .pad(5.),
                                        ])
                                        .height(25.),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
                col_divider(DEMO_GRAY).height(1.),
                row_spaced(
                    10.,
                    vec![
                        column_spaced(
                            10.,
                            vec![
                                draw_label(state.ui, "Edit personal information", 18.),
                                multiline_label(state.ui, info_blurb, 10.).height(50.),
                            ],
                        )
                        .align(Align::TopLeading)
                        .width_range((80.0)..),
                        column_spaced(
                            5.,
                            vec![
                                draw_label_color(state.ui, "Edit username", 12., DEMO_FG_SECONDARY),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    draw_label_color(state.ui, "@UserName", 12., DEMO_FG)
                                        .x_align(XAlign::Leading)
                                        .pad(5.),
                                ])
                                .height(25.),
                                draw_label_color(state.ui, "Bio", 12., DEMO_FG_SECONDARY),
                                stack(vec![
                                    rect(DEMO_FG, DEMO_BG, 5.),
                                    draw_label_color(
                                        state.ui,
                                        "Living, laughing, loving",
                                        12.,
                                        DEMO_FG,
                                    )
                                    .align(Align::TopLeading)
                                    .pad(5.),
                                ])
                                .height(50.),
                            ],
                        )
                        .x_align(XAlign::Leading),
                    ],
                )
                .y_align(YAlign::Top),
            ],
        )
        .align(Align::TopLeading)
        .pad_y(40.)
        .pad_x(30.),
        rect_stroke(DEMO_GRAY),
    ])
    .pad(20.)])
}

fn side_bar<'a>(state: &mut State<'_>) -> Node<State<'a>> {
    stack(vec![
        rect(Color32::TRANSPARENT, DEMO_BG, 0.),
        column_spaced(
            15.,
            vec![
                row_spaced(
                    10.,
                    vec![
                        menu_button(state),
                        draw_label(state.ui, "BACKER", 22.).height(35.),
                    ],
                ),
                col_divider(DEMO_GRAY).pad_x(-30.).height(1.),
                draw_label(state.ui, "Home", 10.),
                draw_label(state.ui, "Explore", 10.),
                draw_label(state.ui, "Marketplace", 10.),
                draw_label(state.ui, "My Account", 10.),
                col_divider(DEMO_GRAY).pad_trailing(-20.).height(1.),
                draw_label(state.ui, "Activity", 10.),
                draw_label(state.ui, "News", 10.),
                draw_label(state.ui, "Docs", 10.),
                col_divider(DEMO_GRAY).pad_trailing(-20.).height(1.),
                draw_label(state.ui, "Twitter", 10.),
                draw_label(state.ui, "Telegram", 10.),
                draw_label(state.ui, "Medium", 10.),
                space(),
            ],
        )
        .align(Align::TopLeading)
        .pad(30.),
    ])
    .x_align(XAlign::Leading)
    .width(200.)
}

fn header<'a>(state: &mut State<'_>) -> Node<State<'a>> {
    row_spaced(
        10.,
        vec![
            menu_button(state),
            draw_label(state.ui, "My Account", 18.)
                .y_align(YAlign::Bottom)
                .width(110.),
            space(),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                draw_label(state.ui, "$115,000", 12.),
            ])
            .width(80.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                row(vec![draw_label(state.ui, "Operational", 12.)]),
            ])
            .width(90.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/bell.svg")).pad_y(8.5),
            ])
            .aspect(1.),
            stack(vec![
                rect(DEMO_FG, DEMO_HINT, 5.),
                icon(include_image!("../assets/user.svg")).pad_y(8.5),
            ])
            .aspect(1.),
        ],
    )
    .pad_top(35.)
    .pad_bottom(15.)
    .pad_x(30.)
    .height(80.)
}

fn menu_button<'a>(_ui: &mut State<'_>) -> Node<State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        if ui
            .ui
            .put(
                rect_from(area),
                Button::image(include_image!("../assets/menu-scale.svg"))
                    .fill(Color32::TRANSPARENT),
            )
            .clicked()
        {
            *ui.sidebar = !*ui.sidebar;
        }
    })
    .aspect(1.)
    .width(30.)
    .height(30.)
}

fn icon<'a>(image: impl Into<ImageSource<'static>> + 'static) -> Node<State<'a>> {
    let image = Image::new(image).tint(Color32::WHITE);
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.put(rect_from(area), image.clone());
    })
}

fn multiline_label<'a, S: AsRef<str> + 'static>(
    _ui: &'_ mut Ui,
    text: S,
    size: f32,
) -> Node<State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        let layout_job = LayoutJob::single_section(
            text.as_ref().to_string(),
            egui::TextFormat {
                font_id: egui::FontId::new(size, egui::FontFamily::Proportional),
                extra_letter_spacing: 0.,
                line_height: Some(14.),
                color: Color32::WHITE,
                background: Color32::TRANSPARENT,
                italics: false,
                underline: Stroke::NONE,
                strikethrough: Stroke::NONE,
                valign: EguiAlign::Min,
            },
        );
        let rect = rect_from(area);
        ui.ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical(|ui| {
                ui.add(Label::new(layout_job));
            })
        });
    })
}

fn draw_label<'a, S: AsRef<str> + 'static>(ui: &'_ mut Ui, text: S, size: f32) -> Node<State<'a>> {
    draw_label_color(ui, text, size, DEMO_FG)
}

fn draw_label_color<'a, S: AsRef<str> + 'static>(
    ui: &'_ mut Ui,
    text: S,
    size: f32,
    color: Color32,
) -> Node<State<'a>> {
    let job = LayoutJob::simple(
        text.as_ref().to_string(),
        egui::FontId::new(size, egui::FontFamily::Proportional),
        color,
        200.,
    );
    let label = egui::Label::new(job);
    let galley = label.layout_in_ui(ui).1.rect;
    let text_area = area_from(galley);
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.put(
            rect_from(area),
            egui::Label::new(LayoutJob::simple(
                text.as_ref().to_string(),
                egui::FontId::new(size, egui::FontFamily::Proportional),
                color,
                200.,
            )),
        );
    })
    .width(text_area.width)
    .height(text_area.height)
}

fn col_divider<'a>(color: Color32) -> Node<State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.painter().line_segment(
            [
                Pos2::new(area.x, area.y + (area.height * 0.5)),
                Pos2::new(area.x + area.width, area.y + (area.height * 0.5)),
            ],
            Stroke::new(1., color),
        );
    })
}
fn row_divider<'a>(color: Color32) -> Node<State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui.painter().line_segment(
            [
                Pos2::new(area.x + (area.width * 0.5), area.y),
                Pos2::new(area.x + (area.width * 0.5), area.y + area.height),
            ],
            Stroke::new(1., color),
        );
    })
}

fn rect<'a>(stroke: Color32, fill: Color32, rounding: f32) -> Node<State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui
            .painter()
            .rect_stroke(rect_from(area), rounding, Stroke::new(1., stroke));
        ui.ui.painter().rect_filled(rect_from(area), rounding, fill);
    })
}

fn rect_stroke<'a>(color: Color32) -> Node<State<'a>> {
    draw(move |area, ui: &mut State<'_>| {
        ui.ui
            .painter()
            .rect_stroke(rect_from(area), 5., Stroke::new(1., color));
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
