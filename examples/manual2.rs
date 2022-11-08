use snake_solver::{
	snake::{Direction, SnakeWorld},
	solvers::{snake_spanning_tree::SnakeSpanningTreeSolver, SnakeSolver},
	ui::SnakeWorldViewer,
};

use eframe::egui::{self};

const GRID_SIZE: i32 = 20;

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
		Box::new(|_cc| Box::new(MyApp::new(SnakeSpanningTreeSolver::new(GRID_SIZE as usize)))),
	);
}

struct MyApp<SS: SnakeSolver> {
	world: SnakeWorld,
	solver: SS,
	speed: u32,
}

impl<SS: SnakeSolver> MyApp<SS> {
	fn new(solver: SS) -> MyApp<SS> {
		Self {
			world: SnakeWorld::new(GRID_SIZE as usize),
			solver,
			speed: 1,
		}
	}
}

impl<SS: SnakeSolver> eframe::App for MyApp<SS> {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			if ctx.input().key_pressed(egui::Key::ArrowUp) {
				self.world.step_snake(Direction::Up);
			} else if ctx.input().key_pressed(egui::Key::ArrowDown) {
				self.world.step_snake(Direction::Down);
			} else if ctx.input().key_pressed(egui::Key::ArrowLeft) {
				self.world.step_snake(Direction::Left);
			} else if ctx.input().key_pressed(egui::Key::ArrowRight) {
				self.world.step_snake(Direction::Right);
			}

			if ctx.input().key_pressed(egui::Key::Space) {
				self.solver.get_next_path(&self.world);
			}

			let mut widget = SnakeWorldViewer::new(&self.world);

			widget = self.solver.decorate_widget(widget);

			ui.add(widget);

			ui.horizontal(|ui| {
				ui.label("Speed");
				ui.add(egui::Slider::new(&mut self.speed, 1..=10000));
			});

			ctx.request_repaint();
		});
	}
}
