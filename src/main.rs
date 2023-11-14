use eframe::egui::{Context, Sense};
use eframe::{egui, Frame};
use path_finding::frame_history::FrameHistory;
use path_finding::node::{Node, NodeType};
use path_finding::Grid;

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(MyApp::build(10, 10))),
            )
            .await
            .expect("failed to start app");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let icon_data = eframe::IconData::try_from_png_bytes(include_bytes!("assets/icon.png")).ok();
    let options = eframe::NativeOptions {
        icon_data,
        ..Default::default()
    };
    eframe::run_native(
        "astar.rs",
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

#[derive(Debug, PartialEq)]
enum Heuristic {
    Manhattan,
    Diagonal,
    Euclidean,
}

trait NodeColor {
    fn get_color(&self) -> egui::Color32;
}

impl NodeColor for Node {
    fn get_color(&self) -> egui::Color32 {
        match self.node_type {
            NodeType::Obstacle => egui::Color32::LIGHT_RED,
            NodeType::Traversable => egui::Color32::TRANSPARENT,
            NodeType::Traversed => egui::Color32::LIGHT_GRAY,
            NodeType::Path => egui::Color32::GRAY,
        }
    }
}

const WIDGET_SPACING: f32 = 10.0;

struct MyApp {
    height: usize,
    width: usize,
    stroke: egui::Stroke,
    rounding: egui::Rounding,
    grid: Grid,
    cursor_type: CursorType,
    frame_history: FrameHistory,
    new_height: usize,
    new_width: usize,
    show_cost: bool,
    heuristic: Heuristic,
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
            frame_history: FrameHistory::default(),
            new_height: height,
            new_width: width,
            show_cost: true,
            heuristic: Heuristic::Manhattan,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        egui::SidePanel::right("my_left_panel").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.add_space(WIDGET_SPACING);
                    ui.label("SETTINGS");
                    ui.add_space(WIDGET_SPACING);
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
                    ui.add_space(WIDGET_SPACING);
                    ui.group(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.new_height, 2..=50)
                                .text("Grid height")
                                .integer(),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.new_width, 2..=50)
                                .text("Grid width")
                                .integer(),
                        );
                        if ui.button("Rebuild Grid").clicked() {
                            self.height = self.new_height;
                            self.width = self.new_width;
                            self.grid = Grid::new(self.new_height, self.new_width);
                        }
                    });
                    ui.add_space(WIDGET_SPACING);
                    ui.label("Heuristic");
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.heuristic,
                                Heuristic::Manhattan,
                                "Manhattan",
                            );
                            ui.selectable_value(
                                &mut self.heuristic,
                                Heuristic::Diagonal,
                                "Diagonal",
                            );
                            ui.selectable_value(
                                &mut self.heuristic,
                                Heuristic::Euclidean,
                                "Euclidean",
                            );
                        });
                    });
                    ui.end_row();
                    ui.add_space(WIDGET_SPACING);
                    ui.checkbox(&mut self.grid.allow_diagonal, "Move Diagonally");
                    ui.add_space(WIDGET_SPACING);
                    ui.checkbox(&mut self.show_cost, "Show Cost");
                    ui.add_space(WIDGET_SPACING);
                    ui.add_enabled_ui(self.grid.is_ready(), |ui| {
                        if ui.button("Find Path").clicked() {
                            self.grid.solve();
                        }
                    });
                    ui.separator();
                    ui.label(format!("FPS: {:.1}", self.frame_history.fps()));
                    if let Some(duration) = self.grid.duration {
                        ui.label(format!("Completed in: {:.1} μs", duration.as_micros()));
                    }
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
                        ui.add_visible_ui(self.show_cost, |ui| {
                            let should_display_cost = self.grid.get_node_at(x, y).node_type
                                != NodeType::Traversable
                                && self.grid.get_node_at(x, y).node_type != NodeType::Obstacle;
                            if should_display_cost {
                                let node = self.grid.get_node_at(x, y);
                                let f_cost = node.f_cost;
                                let g_cost = node.g_cost;
                                let h_cost = node.h_cost;
                                ui.colored_label(egui::Color32::BLACK, format!("{g_cost}"));
                                ui.label(
                                    egui::RichText::new(format!("{f_cost}"))
                                        .font(egui::FontId::proportional(20.0))
                                        .color(egui::Color32::BLACK),
                                );
                                ui.colored_label(egui::Color32::BLACK, format!("{h_cost}"));
                            }
                        });
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
