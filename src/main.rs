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

enum Type {
    Goal,
    Obstacle,
    Start,
}

#[derive(Clone)]
struct Node {
    color: egui::Color32,
}
impl Node {
    fn set_color(&mut self) {
        self.color = egui::Color32::LIGHT_RED;
    }
}
impl Default for Node {
    fn default() -> Self {
        Node {
            color: egui::Color32::TRANSPARENT,
        }
    }
}
struct MyApp {
    height: usize,
    width: usize,
    stroke: egui::Stroke,
    rounding: egui::Rounding,
    grid: Vec<Vec<Node>>,
}

impl MyApp {
    fn build(width: usize, height: usize) -> Self {
        let grid = vec![vec![Node::default(); width]; height];

        MyApp {
            width,
            height,
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_RED),
            rounding: egui::Rounding::default(),
            grid,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::right("my_left_panel").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.label("Settings");
                    ui.button("Find Path")
                },
            )
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_size = ui.available_size();
            // compute the rect_size
            let rect_size = egui::Vec2::new(
                (panel_size.x - 20.0) / self.width as f32,
                (panel_size.y - 20.0) / self.height as f32,
            );
            let (_, painter) = ui.allocate_painter(panel_size, Sense::click());
            for y in 0..self.height {
                for x in 0..self.width {
                    let x_coord = x as f32 * rect_size.x + 10.0;
                    let y_coord = y as f32 * rect_size.y + 10.0;
                    let pos = egui::pos2(x_coord, y_coord);
                    let rect = egui::Rect::from_min_size(pos, rect_size);
                    painter.rect_filled(rect, self.rounding, self.grid[x][y].color);
                    painter.rect_stroke(rect, self.rounding, self.stroke);
                    ui.allocate_ui_at_rect(rect, |ui| {
                        let (_, res) = ui.allocate_exact_size(rect_size, Sense::click());
                        if res.clicked() {
                            self.grid[x][y].set_color();
                        }
                    });
                }
            }
        });
    }
}
