use std::f32::consts::PI;

use eframe::{
    App, Frame,
    egui::{self, Color32, Pos2, Shape, Stroke},
};

pub struct HeartApp;

impl App for HeartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let to_screen = |x: f32, y: f32, width: f32, height: f32| -> Pos2 {
                let scale = 100.0;
                Pos2::new(width / 2.0 + x * scale, height / 2.0 - y * scale)
            };

            let width = response.rect.width();
            let height = response.rect.height();

            let mut points = Vec::new();
            let steps = 1000;
            for i in 0..=steps {
                let t = i as f32 / steps as f32 * 2.0 * PI;
                let sin_t = t.sin();
                let cos_t = t.cos();

                let x = (2.0f32).sqrt() * sin_t.powi(3);
                let y = -cos_t.powi(3) - cos_t.powi(2) + 2.0 * cos_t;

                points.push(to_screen(x, y, width, height));
            }

            painter.add(Shape::line(points, Stroke::new(2.0, Color32::RED)));
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Heart Shape with Parametric Formula",
        options,
        Box::new(|_cc| Ok(Box::new(HeartApp))),
    )
}
