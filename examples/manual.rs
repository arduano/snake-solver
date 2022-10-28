use snake_solver::{snake::SnakeWorld, ui::SnakeWorldViewer, *};

use eframe::egui::{self};

const GRID_SIZE: i32 = 80;

fn main() {
    let width: f32 = GRID_SIZE as f32 * 10.0 + 20.0;
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(width, width)),
        ..Default::default()
    };

    eframe::run_native(
        "Manual snake game",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    snake_world: SnakeWorld,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            snake_world: SnakeWorld::new(GRID_SIZE as usize),
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

        egui::CentralPanel::default()
            .show(ctx, |ui| ui.add(SnakeWorldViewer::new(&self.snake_world)));
    }
}
