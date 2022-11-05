use snake_solver::{
	auto::{AutoPlayerState, AutoSnakePlayer},
	solvers::{basic::BasicSnakeSolver, SnakeSolver},
	ui::SnakeWorldViewer,
};

use eframe::egui::{self};

const GRID_SIZE: i32 = 80;
type CurrentSnakeSolver = BasicSnakeSolver;

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
		Box::new(|_cc| Box::new(MyApp::<CurrentSnakeSolver>::default())),
	);
}

struct MyApp<SS: SnakeSolver> {
	world: AutoSnakePlayer<SS>,
	speed: u32,
}

impl<SS: SnakeSolver> Default for MyApp<SS> {
	fn default() -> Self {
		Self {
			world: AutoSnakePlayer::new(GRID_SIZE as usize),
			speed: 1,
		}
	}
}

impl<SS: SnakeSolver> eframe::App for MyApp<SS> {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			match self.world.state() {
				AutoPlayerState::Playing => {
					ui.add(
						SnakeWorldViewer::new(&self.world.world())
							.with_path_overlay(self.world.current_path()),
					);
				}
				AutoPlayerState::Finished => {
					ui.heading("Finished");
				}
				AutoPlayerState::Killed => {
					ui.heading("Killed");
				}
			}

			ui.horizontal(|ui| {
				ui.label("Speed");
				ui.add(egui::Slider::new(&mut self.speed, 1..=10000));
			});

			for _ in 0..self.speed {
				self.world.step();
			}

			ctx.request_repaint();
		});
	}
}
