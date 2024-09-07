use backer::{
    layout::{Layout, Node},
    models::{Area, Size, XAlign},
    nodes::{draw, stack},
};
use eframe::egui;
use egui::{Color32, Pos2, Rect, RichText, Stroke, Ui};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Layout Example", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout { tree };
            let viewport = ctx.input(|i| i.screen_rect());
            let available_area = area_from(viewport);
            layout.draw(available_area, ui);
        });
    })
}

fn tree(ui: &mut Ui) -> Node<Ui> {
    stack(vec![
        draw_rect(Color32::from_rgb(255, 255, 255), true),
        draw_label(ui, "My Label".to_string()).pad(10.),
    ])
    .pad(5.)
}

fn draw_label(ui: &mut Ui, text: String) -> Node<Ui> {
    let label = egui::Label::new(RichText::new(text.clone()).size(16.));
    let galley = label.layout_in_ui(ui).1.rect;
    let text_area = area_from(galley);
    draw(move |area, ui: &mut Ui| {
        ui.put(
            rect(area),
            egui::Label::new(RichText::new(text.clone()).size(16.)),
        );
    })
    .size(
        Size::new()
            .width(text_area.width)
            .height(text_area.height)
            .align(backer::models::Align::TopLeading),
    )
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
