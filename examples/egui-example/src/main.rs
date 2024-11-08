use backer::Layout;
use backer::Node;
use backer::{models::*, nodes::*};
use eframe::egui;

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
    // stack(vec![
    //     draw_a(ui)
    //         .height(30.)
    //         .width(20.)
    //         // .aspect(1.)
    //         .expand_y()
    //         .align(Align::BottomTrailing),
    //     draw_b(ui).height(30.).width(30.).align(Align::TopLeading),
    // ])
    // .width(100.)
    // .expand_y()
    // .attach_over(draw_rect(Color32::GREEN, true))

    // column(vec![draw_a(ui).aspect(0.5).align(Align::Leading)]).expand()

    // column(vec![draw_a(ui).width(10.), draw_a(ui).width(30.)])
    //     .align(Align::Leading)
    //     .expand()

    // draw_a(ui).align(Align::Leading).aspect(0.5)

    // .expand()
    // .width(200.)
    // .align(Align::Leading)
    // .expand()

    // draw_a(ui).expand().aspect(0.5).align(Align::Leading)

    // row(vec![
    //     row(vec![draw_b(ui).width(10.)]),
    //     draw_a(ui).width(30.),
    // ])
    // .align_contents(Align::Trailing)
    // .expand()

    draw_a(ui)
        .width_range(20.0..)
        .pad(0.)
        .attach_over(draw_c(ui))
        .width_range(..10.)
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
    stack(vec![draw_rect(color, true), draw_label(ui, text)])
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
