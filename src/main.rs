use eframe::egui::{Context, Sense};
use eframe::{egui, Frame};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Path Finding",
        options,
        Box::new(|_cc| Box::new(MyApp::build(10, 10))),
    )
    .expect("failed to initialise app")
}

struct MyApp {
    height: u32,
    width: u32,
    stroke: egui::Stroke,
    rounding: egui::Rounding,
}

impl MyApp {
    fn build(width: u32, height: u32) -> Self {
        MyApp {
            width,
            height,
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_RED),
            rounding: egui::Rounding::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_size = frame.info().window_info.size;
            // compute the rect_size
            let rect_size = egui::Vec2::new(
                (window_size.x - 20.0) / self.width as f32,
                (window_size.y - 20.0) / self.height as f32,
            );
            let (_, painter) = ui.allocate_painter(window_size, Sense::hover());
            for y in 0..self.height {
                for x in 0..self.width {
                    let x_coord = x as f32 * rect_size.x + 10.0;
                    let y_coord = y as f32 * rect_size.y + 10.0;
                    let pos = egui::pos2(x_coord, y_coord);
                    let rect = egui::Rect::from_min_size(pos, rect_size);
                    painter.rect_stroke(rect, self.rounding, self.stroke);
                }
            }
        });
    }
}
