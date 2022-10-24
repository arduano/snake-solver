mod array2d;
mod snake;

use eframe::egui::{self, Sense};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    snake_world: snake::SnakeWorld,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            snake_world: snake::SnakeWorld::new(200),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sense arrow keys
        if ctx.input().key_pressed(egui::Key::ArrowUp) {
            self.snake_world.step_snake(snake::Direction::Up);
        } else if ctx.input().key_pressed(egui::Key::ArrowDown) {
            self.snake_world.step_snake(snake::Direction::Down);
        } else if ctx.input().key_pressed(egui::Key::ArrowLeft) {
            self.snake_world.step_snake(snake::Direction::Left);
        } else if ctx.input().key_pressed(egui::Key::ArrowRight) {
            self.snake_world.step_snake(snake::Direction::Right);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), Sense::click());

            let mut mesh = egui::Mesh::default();
            for x in 0..self.snake_world.size() {
                for y in 0..self.snake_world.size() {
                    let coord = array2d::Coord::new(x, y);
                    let cell = self.snake_world.get_cell(coord).copied();
                    let rect = egui::Rect::from_min_size(
                        rect.min + egui::vec2(x as f32 * 10.0, y as f32 * 10.0),
                        egui::vec2(10.0, 10.0),
                    );
                    match cell {
                        Some(snake::Cell::Empty) => {
                            mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 0, 0));
                        }
                        Some(snake::Cell::Snake(_)) => {
                            mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 255, 0));
                        }
                        Some(snake::Cell::Food) => {
                            mesh.add_colored_rect(rect, egui::Color32::from_rgb(255, 0, 0));
                        }

                        None => unreachable!(),
                    }
                }
            }

            let painter = ui.painter();
            painter.add(egui::Shape::Mesh(mesh));
        });
    }
}
