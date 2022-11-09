use snake_solver::{direction::Direction, snake::SnakeWorld, ui::SnakeWorldViewer};

use eframe::egui::{self};

const GRID_SIZE: i32 = 80;

fn main() {
	let width: f32 = SnakeWorldViewer::calculate_size_for_world_size(GRID_SIZE as usize) + 20.0;
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
			self.snake_world.step_snake(Direction::Up);
		} else if ctx.input().key_pressed(egui::Key::ArrowDown) {
			self.snake_world.step_snake(Direction::Down);
		} else if ctx.input().key_pressed(egui::Key::ArrowLeft) {
			self.snake_world.step_snake(Direction::Left);
		} else if ctx.input().key_pressed(egui::Key::ArrowRight) {
			self.snake_world.step_snake(Direction::Right);
		}

		egui::CentralPanel::default()
			.show(ctx, |ui| ui.add(SnakeWorldViewer::new(&self.snake_world)));
	}
}
