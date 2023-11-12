use eframe::egui::{Color32, Context, Sense};
use eframe::{egui, Frame};
use path_finding::node::{Node, NodeType};
use path_finding::Grid;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Path Finding",
        options,
        Box::new(|_cc| Box::new(MyApp::build(10, 10))),
    )
    .expect("failed to initialise app")
}

#[derive(Debug, PartialEq)]
enum CursorType {
    Goal,
    Obstacle,
    Start,
}

trait NodeColor {
    fn get_color(&self) -> egui::Color32;
}

impl NodeColor for Node {
    fn get_color(&self) -> Color32 {
        match self.node_type {
            NodeType::Obstacle => egui::Color32::LIGHT_RED,
            NodeType::Traversable => egui::Color32::TRANSPARENT,
            NodeType::Traversed => egui::Color32::DARK_GRAY,
        }
    }
}

struct MyApp {
    height: usize,
    width: usize,
    stroke: egui::Stroke,
    rounding: egui::Rounding,
    grid: Grid,
    cursor_type: CursorType,
}

impl MyApp {
    fn build(width: usize, height: usize) -> Self {
        let grid = Grid::new(height, width);
        MyApp {
            width,
            height,
            stroke: egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
            rounding: egui::Rounding::default(),
            grid,
            cursor_type: CursorType::Start,
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
                    ui.end_row();
                    egui::ComboBox::from_label("Select Node Type")
                        .selected_text(format!("{:?}", self.cursor_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.cursor_type, CursorType::Start, "Start");
                            ui.selectable_value(
                                &mut self.cursor_type,
                                CursorType::Obstacle,
                                "Obstacle",
                            );
                            ui.selectable_value(&mut self.cursor_type, CursorType::Goal, "Goal");
                        });
                    ui.end_row();
                    ui.add_enabled_ui(self.grid.is_ready(), |ui| {
                        if ui.button("Find Path").clicked() {
                            self.grid.solve();
                        }
                    })
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
                    let mut color = self.grid.get_node_at(x, y).get_color();
                    if self.grid.is_goal(x, y) {
                        color = egui::Color32::LIGHT_GREEN;
                    }
                    if self.grid.is_start(x, y) {
                        color = egui::Color32::LIGHT_BLUE;
                    }
                    painter.rect_filled(rect, self.rounding, color);
                    painter.rect_stroke(rect, self.rounding, self.stroke);
                    ui.allocate_ui_at_rect(rect, |ui| {
                        let (_, res) = ui.allocate_exact_size(rect_size, Sense::click());
                        if res.clicked() {
                            match self.cursor_type {
                                CursorType::Goal => self.grid.set_goal(x, y),
                                CursorType::Obstacle => self.grid.set_obstacle(x, y),
                                CursorType::Start => self.grid.set_start(x, y),
                            }
                        }
                    });
                }
            }
        });
    }
}
