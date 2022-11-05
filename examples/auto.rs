use snake_solver::{
	auto::{AutoPlayerState, AutoSnakePlayer},
	solvers::{random_spanning_tree::RandomSpanningTreeSolver, SnakeSolver},
	ui::SnakeWorldViewer,
};

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
		Box::new(|_cc| Box::new(MyApp::new(RandomSpanningTreeSolver::new()))),
	);
}

struct MyApp<SS: SnakeSolver> {
	world: AutoSnakePlayer<SS>,
	speed: u32,
}

impl<SS: SnakeSolver> MyApp<SS> {
	fn new(solver: SS) -> MyApp<SS> {
		Self {
			world: AutoSnakePlayer::new(GRID_SIZE as usize, solver),
			speed: 1,
		}
	}
}

impl eframe::App for MyApp<RandomSpanningTreeSolver> {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			match self.world.state() {
				AutoPlayerState::Playing | AutoPlayerState::Killed => {
					let mut widget = SnakeWorldViewer::new(&self.world.world());

					widget = widget.with_path_overlay(self.world.current_path());

					if let Some(grid) = &self.world.solver.prev_grid {
						widget = widget.with_bools_edges_grid_overlay(grid);
					}

					ui.add(widget);
				}
				AutoPlayerState::Finished => {
					ui.heading("Finished");
				}
			}

			ui.horizontal(|ui| {
				ui.label("Speed");
				ui.add(egui::Slider::new(&mut self.speed, 1..=10000));
			});

			if ctx.input().key_pressed(egui::Key::Space) {
				for _ in 0..self.speed {
					self.world.step();
				}
			}

			ctx.request_repaint();
		});
	}
}
