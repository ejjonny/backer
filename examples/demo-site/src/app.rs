use backer::{models::*, nodes::*, Layout, Node};
use egui::{
    include_image, Button, Color32, Image, ImageSource, Pos2, Rect, RichText, Stroke, TextFormat,
    Ui,
};

#[derive(Default)]
pub struct TemplateApp {}

impl TemplateApp {
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout::new(my_layout_fn);
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            layout.draw(available_area, ui);
        });
    }
}

const DEMO_GRAY: Color32 = Color32::from_rgb(50, 50, 50);

fn my_layout_fn(ui: &mut Ui) -> Node<Ui> {
    row(vec![
        column_spaced(
            20.,
            vec![
                draw_label(ui, RichText::new("MAVIA").color(Color32::WHITE).size(22.)).height(30.),
                col_divider(DEMO_GRAY).pad_x(-30.).height(1.),
                draw_label(ui, RichText::new("s Home")),
                draw_label(ui, RichText::new("• Explore")),
                draw_label(ui, RichText::new("# Marketplace")),
                draw_label(ui, RichText::new("• My Account")),
                col_divider(DEMO_GRAY).pad_trailing(-30.).height(1.),
                draw_label(ui, RichText::new("• Activity")),
                draw_label(ui, RichText::new("! News")),
                draw_label(ui, RichText::new("• Docs")),
                col_divider(DEMO_GRAY).pad_trailing(-30.).height(1.),
                draw_label(ui, RichText::new("y Twitter")),
                draw_label(ui, RichText::new("A Telegram")),
                draw_label(ui, RichText::new("M Medium")),
                space(),
            ],
        )
        .align(Align::TopLeading)
        .pad(30.)
        .width(200.),
        row_divider(DEMO_GRAY).width(1.),
        column_spaced(
            20.,
            vec![
                row_spaced(
                    10.,
                    vec![
                        draw_label(
                            ui,
                            RichText::new("My Account").color(Color32::WHITE).size(22.),
                        ),
                        space(),
                        stack(vec![
                            draw_label(
                                ui,
                                RichText::new("$115,000").color(Color32::WHITE).size(12.),
                            ),
                            rect_stroke(DEMO_GRAY),
                        ])
                        .width(80.),
                        stack(vec![
                            row(vec![draw_label(
                                ui,
                                RichText::new("Operational").color(Color32::WHITE).size(12.),
                            )]),
                            rect_stroke(DEMO_GRAY),
                        ])
                        .width(90.),
                        stack(vec![
                            icon(include_image!("../assets/bell.svg"))
                                .aspect(1.)
                                .pad_y(8.5)
                                .aspect(1.),
                            rect_stroke(DEMO_GRAY),
                        ])
                        .aspect(1.),
                        stack(vec![
                            icon(include_image!("../assets/user.svg"))
                                .aspect(1.)
                                .pad_y(8.5)
                                .aspect(1.),
                            rect_stroke(DEMO_GRAY),
                        ])
                        .aspect(1.),
                    ],
                )
                .height(30.),
                col_divider(DEMO_GRAY).pad_x(-30.).height(1.),
                stack(vec![
                    row(vec![
                        column(vec![draw_label(
                            ui,
                            RichText::new("Public Profile").size(18.),
                        ),
                        draw_label(
                            ui,
                            RichText::new("Your public profile URL can be shared with anyone and allows them to immediately see your bases and activity in Mavia."
                                ).size(9.),
                        ).width(200.)])
                        .align(Align::TopLeading)
                        .pad_y(40.)
                        .pad_x(30.),
                        rect_stroke(DEMO_GRAY),
                    ]),
                    rect_stroke(DEMO_GRAY),
                ])
                .pad(8.),
            ],
        )
        .align(Align::TopLeading)
        .pad(30.),
    ])
}

// Public Profile
// Your public profile URL can be shared with anyone and allows them to immediately see your bases and activity in Mavia.
// Edit profile picture
// Upload a profile picture of yourself, or the character, you always wanted to be. Your avatar will be displayes all over the Mavia world.
// Edit personal information
// Tell the world aboit yourself. Information you add, will bo visible only in your profile, nut for all users, who will visit your profile.
fn icon(image: impl Into<ImageSource<'static>> + 'static) -> Node<Ui> {
    let image = Image::new(image).tint(Color32::WHITE);
    draw(move |area, ui: &mut Ui| {
        ui.put(rect_from(area), image.clone());
    })
}

fn button(action: fn(&mut Ui)) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        if ui.put(rect_from(area), Button::new("Backer Off")).clicked() {
            action(ui);
        }
    })
}

fn draw_label(ui: &'_ mut Ui, text: RichText) -> Node<Ui> {
    let label = egui::Label::new(text.clone());
    let galley = label.layout_in_ui(ui).1.rect;
    let text_area = area_from(galley);
    draw(move |area, ui: &mut Ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.put(rect_from(area), egui::Label::new(text.clone()));
        });
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

fn rect_stroke(color: Color32) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        ui.painter()
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
