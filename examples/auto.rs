use snake_solver::{auto::AutoSnakePlayer, solvers::basic::BasicSnakeSolver, ui::SnakeWorldViewer};

use eframe::egui::{self};

const GRID_SIZE: i32 = 80;

fn main() {
    let width: f32 = GRID_SIZE as f32 * 10.0 + 20.0;

    let extra_height = 20.0;

    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(width, width + extra_height)),
        ..Default::default()
    };

    eframe::run_native(
        "Auto snake game",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    world: AutoSnakePlayer<BasicSnakeSolver>,
    speed: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            world: AutoSnakePlayer::new(GRID_SIZE as usize),
            speed: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                SnakeWorldViewer::new(&self.world.world())
                    .with_path_overlay(self.world.current_path()),
            );

            ui.horizontal(|ui| {
                ui.label("Speed");
                ui.add(egui::Slider::new(&mut self.speed, 1..=10000).text("speed"));
            });

            for _ in 0..self.speed {
                self.world.step();
            }

            ctx.request_repaint();
        });
    }
}
