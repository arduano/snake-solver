use std::fmt::{Display, Formatter};

use snake_solver::{
	auto::{AutoPlayerState, AutoSnakePlayer},
	solvers::{
		basic::BasicSnakeSolver,
		random_spanning_tree::RandomSpanningTreeSolver,
		snake_spanning_tree::{JitterKind, SnakeSpanningTreeSolver},
		SnakeSolver,
	},
	ui::SnakeWorldViewer,
};

use eframe::egui::{self};

#[derive(Debug)]
enum Solvers {
	ZigZag,
	StaticHamiltonian,
	DynamicHamiltonian,
	DynamicHamiltonian10Jitter,
	DynamicHamiltonian1Jitter,
}

impl Display for Solvers {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Solvers::ZigZag => write!(f, "Zig-Zag"),
			Solvers::StaticHamiltonian => write!(f, "Static Hamiltonian"),
			Solvers::DynamicHamiltonian => write!(f, "Dynamic Hamiltonian"),
			Solvers::DynamicHamiltonian10Jitter => {
				write!(f, "Dynamic Hamiltonian (10-step jitter)")
			}
			Solvers::DynamicHamiltonian1Jitter => write!(f, "Dynamic Hamiltonian (1-step jitter)"),
		}
	}
}

fn main() {
	let grid_size = dialoguer::Input::new()
		.with_prompt("World size (e.g. 40 or 60)")
		.validate_with(|str: &String| match str.parse::<usize>() {
			Ok(_) => Ok(()),
			Err(_) => Err("Please enter a number".to_string()),
		})
		.interact()
		.unwrap()
		.parse::<usize>()
		.unwrap();

	let solvers = [
		Solvers::ZigZag,
		Solvers::StaticHamiltonian,
		Solvers::DynamicHamiltonian,
		Solvers::DynamicHamiltonian10Jitter,
		Solvers::DynamicHamiltonian1Jitter,
	];

	let solver_index = dialoguer::Select::new()
		.with_prompt("Pick which solving algorithm should be used")
		.items(&solvers)
		.interact()
		.unwrap();
	let solver = &solvers[solver_index];

	match solver {
		Solvers::ZigZag => run(grid_size, BasicSnakeSolver),
		Solvers::StaticHamiltonian => run(grid_size, RandomSpanningTreeSolver::new()),
		Solvers::DynamicHamiltonian => run(
			grid_size,
			SnakeSpanningTreeSolver::new(JitterKind::NoJitter),
		),
		Solvers::DynamicHamiltonian10Jitter => run(
			grid_size,
			SnakeSpanningTreeSolver::new(JitterKind::JitterWhenIndirect(10)),
		),
		Solvers::DynamicHamiltonian1Jitter => run(
			grid_size,
			SnakeSpanningTreeSolver::new(JitterKind::JitterWhenIndirect(1)),
		),
	}
}

fn run(grid_size: usize, solver: impl 'static + SnakeSolver) {
	let width: f32 = SnakeWorldViewer::calculate_size_for_world_size(grid_size) + 20.0;

	let extra_height = 20.0 + 100.0;

	let options = eframe::NativeOptions {
		min_window_size: Some(egui::vec2(width, width + extra_height)),
		..Default::default()
	};

	println!("Opening window... If you don't see the window, it might be behind other windows");

	eframe::run_native(
		"Auto snake game",
		options,
		Box::new(move |_cc| Box::new(MyApp::new(grid_size, solver))),
	);
}

struct MyApp<SS: SnakeSolver> {
	world: AutoSnakePlayer<SS>,
	speed: u32,
	autoplay: bool,
	overlay: bool,
}

impl<SS: SnakeSolver> MyApp<SS> {
	fn new(grid_size: usize, solver: SS) -> MyApp<SS> {
		Self {
			world: AutoSnakePlayer::new(grid_size, solver),
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

			ui.heading("Keyboard controls:");
			ui.label("Spacebar: run 1*speed iterations");
			ui.label("1: run 1*speed iterations");
			ui.label("2: run 10*speed iterations");
			ui.label("3: run 100*speed iterations");
			ui.label("4: run 1000*speed iterations");

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
