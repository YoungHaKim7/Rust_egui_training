#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Keyboard events",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Press/Hold/Release example. Press A to test.");
            if ui.button("Clear").clicked() {
                self.text.clear();
            }
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.label(&self.text);
                });

            if ctx.input(|i| i.key_pressed(Key::A)) {
                self.text.push_str("\n A_Pressed");
            }
            if ctx.input(|i| i.key_pressed(Key::B)) {
                self.text.push_str("\n B_Pressed");
            }
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                self.text.push_str("\n Esc_Pressed");
            }
            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                self.text.push_str("\n Enter_Pressed");
            }
            if ctx.input(|i| i.key_released(Key::A)) {
                self.text.push_str("\n A_Released");
            }
        });
    }
}
