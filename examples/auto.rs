use snake_solver::{
	auto::{AutoPlayerState, AutoSnakePlayer},
	solvers::{snake_spanning_tree::SnakeSpanningTreeSolver, SnakeSolver},
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
		Box::new(|_cc| Box::new(MyApp::new(SnakeSpanningTreeSolver::new(Some(1))))),
	);
}

struct MyApp<SS: SnakeSolver> {
	world: AutoSnakePlayer<SS>,
	speed: u32,
	autoplay: bool,
	overlay: bool,
}

impl<SS: SnakeSolver> MyApp<SS> {
	fn new(solver: SS) -> MyApp<SS> {
		Self {
			world: AutoSnakePlayer::new(GRID_SIZE as usize, solver),
			speed: 1,
			autoplay: false,
			overlay: true,
		}
	}
}

impl<SS: SnakeSolver> eframe::App for MyApp<SS> {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			match self.world.state() {
				AutoPlayerState::Playing | AutoPlayerState::Killed => {
					let mut widget = SnakeWorldViewer::new(&self.world.world());

					if self.overlay {
						widget = widget.with_path_overlay(self.world.current_path());
						widget = self.world.solver.decorate_widget(widget);
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
				ui.add(egui::Checkbox::new(&mut self.autoplay, "Autoplay"));
				ui.add(egui::Checkbox::new(&mut self.overlay, "Overlay"));
			});

			let mut steps = 0;

			if self.autoplay || ctx.input().key_pressed(egui::Key::Space) {
				steps = self.speed as usize;
			}

			if ctx.input().key_pressed(egui::Key::Num1) {
				steps = self.speed as usize;
			}

			if ctx.input().key_pressed(egui::Key::Num2) {
				steps = self.speed as usize * 10;
			}

			if ctx.input().key_pressed(egui::Key::Num3) {
				steps = self.speed as usize * 100;
			}

			if ctx.input().key_pressed(egui::Key::Num4) {
				steps = self.speed as usize * 1000;
			}

			for _ in 0..steps {
				self.world.step();
			}

			ctx.request_repaint();
		});
	}
}
