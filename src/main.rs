use eframe::egui::{Context, Sense};
use eframe::{egui, Frame};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Path Finding",
        options,
        Box::new(|_cc| {
            Box::new(MyApp {
                height: 10,
                width: 10,
            })
        }),
    )
    .expect("failed to initialise app")
}

struct MyApp {
    height: u32,
    width: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_size = frame.info().window_info.size;
            let padding = egui::Vec2::new(10.0, 10.0);
            // compute the rect_size
            let rect_size = egui::Vec2::new(
                (window_size.x - 20.0) / self.width as f32,
                (window_size.y - 20.0) / self.height as f32,
            );
            let (_, painter) = ui.allocate_painter(window_size + padding, Sense::hover());
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos =
                        egui::pos2(x as f32 * rect_size.x + 10.0, y as f32 * rect_size.y + 10.0);
                    let rect = egui::Rect::from_min_size(pos, rect_size);
                    painter.rect_stroke(
                        rect,
                        egui::Rounding::default(),
                        egui::Stroke::new(1.0, egui::Color32::LIGHT_RED),
                    );
                }
            }
        });
    }
}
