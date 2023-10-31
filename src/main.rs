use eframe::egui::Context;
use eframe::{egui, Frame};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Path Finding",
        options,
        Box::new(|_cc| {
            Box::new(MyApp {
                title: "Hello from egui".to_string(),
            })
        }),
    )
    .expect("failed to initialise app")
}

struct MyApp {
    title: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.title);
        });
    }
}
