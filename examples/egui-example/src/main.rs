use backer::models::Align;
use backer::Layout;
use backer::Node;
use backer::{models::*, nodes::*};
use eframe::egui;
use egui::text::LayoutJob;
use egui::Align as EguiAlign;
use egui::Label;
use egui::{Color32, Pos2, Rect, RichText, Stroke, Ui};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Layout Example", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout::new(my_layout_fn);
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            layout.draw(available_area, ui);
        });
    })
}

fn my_layout_fn(ui: &mut Ui) -> Node<Ui> {
    let t = "long text long text long text long text long text long text target/debug/examples/egui-example`
                    2024-09-20 17:23:24.141 egui-example[2388:32853] +[IMKClient subclass]: chose IMKClient_Legacy
                    2024-09-20 17:23:24.141 egui-example[2388:32853] +[IMKInputSession subclass]: chose IMKInputSession_Legacy";
    column_spaced(
        10.,
        (0..6)
            .map(|_| {
                row_spaced(
                    10.,
                    vec![
                        draw_b(ui).aspect(2.),
                        draw_a(ui).width(40.).height(40.).align(Align::TopCenter),
                        column(vec![
                            label_common(t, 10., Color32::WHITE),
                            draw_a(ui).height(20.),
                        ]),
                    ],
                )
            })
            .collect(),
    )
    .y_align(YAlign::Top)
}

fn draw_a(ui: &mut Ui) -> Node<Ui> {
    labeled_rect(ui, "A".to_string(), Color32::BLUE)
}

fn draw_b(ui: &mut Ui) -> Node<Ui> {
    labeled_rect(ui, "B".to_string(), Color32::RED)
}

fn draw_c(ui: &mut Ui) -> Node<Ui> {
    labeled_rect(ui, "C".to_string(), Color32::GOLD)
}

fn labeled_rect(ui: &mut Ui, text: String, color: Color32) -> Node<Ui> {
    // stack(vec![
    draw_rect(color, true)
    // , draw_label(ui, text)
    // ])
}

fn draw_label(ui: &mut Ui, text: String) -> Node<Ui> {
    let label = egui::Label::new(RichText::new(text.clone()).size(10.));
    let galley = label.layout_in_ui(ui).1.rect;
    let text_area = area_from(galley);
    draw(move |area, ui: &mut Ui| {
        ui.put(
            rect(area),
            egui::Label::new(RichText::new(text.clone()).size(10.)),
        );
    })
    .width(text_area.width)
    .height(text_area.height)
}

fn draw_rect(color: Color32, stroke: bool) -> Node<Ui> {
    draw(move |area, ui: &mut Ui| {
        if stroke {
            ui.painter()
                .rect_stroke(rect(area), 5., Stroke::new(3., color));
        } else {
            ui.painter().rect_filled(rect(area), 5., color);
        }
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

fn rect(area: Area) -> Rect {
    Rect {
        min: Pos2::new(area.x, area.y),
        max: Pos2::new(area.x + area.width, area.y + area.height),
    }
}

fn label_common<S: AsRef<str> + 'static + Clone + Copy>(
    text: S,
    size: f32,
    color: Color32,
) -> Node<Ui> {
    fn layout_job(
        font_size: f32,
        width: f32,
        text: String,
        align: EguiAlign,
        color: Color32,
    ) -> LayoutJob {
        let mut job = LayoutJob::single_section(
            text.clone(),
            egui::TextFormat {
                font_id: egui::FontId::new(font_size, egui::FontFamily::Proportional),
                extra_letter_spacing: 0.,
                line_height: Some(14.),
                color,
                background: Color32::TRANSPARENT,
                italics: false,
                underline: Stroke::NONE,
                strikethrough: Stroke::NONE,
                valign: align,
            },
        );
        job.wrap.max_width = width;
        job
    }
    let text = text.as_ref().to_string();
    let text_b = text.clone();
    draw(move |area, ui: &mut Ui| {
        let job = layout_job(size, area.width, text.clone(), EguiAlign::Min, color);
        let rect = rect(area);
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical(|ui| {
                ui.add(Label::new(job.clone()));
            })
        });
    })
    .dynamic_height(move |width, state| {
        let galley_text = text_b.clone();
        let galley_size = state
            .fonts(move |fonts| {
                fonts.layout_job(layout_job(size, width, galley_text, EguiAlign::Min, color))
            })
            .size();
        galley_size.y
    })
}
