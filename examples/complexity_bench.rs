use std::{
	fmt::{Display, Formatter},
	time::{Duration, Instant},
};

use indicatif::ProgressIterator;
use snake_solver::{
	snake::{SnakeResult, SnakeWorld},
	solvers::{
		basic::BasicSnakeSolver,
		random_spanning_tree::RandomSpanningTreeSolver,
		snake_spanning_tree::{JitterKind, SnakeSpanningTreeSolver},
		SnakeSolver,
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoPlayerState {
	Playing,
	Finished,
	Killed,
}

pub fn run<S: SnakeSolver>(world_size: usize, mut solver: S) -> Duration {
	let mut world = SnakeWorld::new(world_size);

	let mut total_duration = Duration::ZERO;
	let mut pathfinds = 0;

	let mut get_next_path = |world: &SnakeWorld| {
		pathfinds += 1;
		let start = Instant::now();
		let path = solver.get_next_path(&world);
		total_duration += start.elapsed();
		path
	};

	let mut path = get_next_path(&world);
	let mut get_next_step = |world: &SnakeWorld| loop {
		if let Some(next) = path.pop() {
			return next;
		} else {
			path = get_next_path(&world);
			if path.is_empty() {
				panic!("Solver returned empty path");
			}
		}
	};

	loop {
		let next_step = get_next_step(&world);
		let result = world.step_snake(next_step);

		if result == SnakeResult::Finished {
			break;
		} else if result == SnakeResult::Killed {
			panic!("Snake died");
		}
	}

	total_duration / pathfinds
}

#[derive(Debug)]
enum Solvers {
	ZigZag,
	StaticHamiltonian,
	DynamicHamiltonian,
}

impl Display for Solvers {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Solvers::ZigZag => write!(f, "Zig-Zag"),
			Solvers::StaticHamiltonian => write!(f, "Static Hamiltonian"),
			Solvers::DynamicHamiltonian => write!(f, "Dynamic Hamiltonian"),
		}
	}
}

pub fn main() {
	let algorithms = [
		Solvers::ZigZag,
		Solvers::StaticHamiltonian,
		Solvers::DynamicHamiltonian,
	];

	let starting_size: usize = 10;
	let up_to: usize = 60;
	let run_count: usize = 10;

	let all_sizes = (starting_size..=up_to).step_by(2).collect::<Vec<_>>();

	let mut results = algorithms
		.iter()
		.map(|_| vec![Duration::ZERO; all_sizes.len()])
		.collect::<Vec<_>>();

	for _ in (0..run_count).progress() {
		for (size_index, &world_size) in all_sizes
			.iter()
			.progress_count(all_sizes.len() as u64)
			.enumerate()
		{
			for (i, algorithm) in algorithms.iter().enumerate() {
				let duration = match algorithm {
					Solvers::ZigZag => run(world_size, BasicSnakeSolver),
					Solvers::StaticHamiltonian => run(world_size, RandomSpanningTreeSolver::new()),
					Solvers::DynamicHamiltonian => run(
						world_size,
						SnakeSpanningTreeSolver::new(JitterKind::JitterWhenIndirect(10)),
					),
				};

				results[i][size_index] += duration;
			}
		}
	}

	// Print the results as a csv
	println!(
		"World Size,{}",
		algorithms
			.iter()
			.map(|a| a.to_string())
			.collect::<Vec<_>>()
			.join(",")
	);

	for (i, _) in results[0].iter().enumerate() {
		let world_size = i * 2 + starting_size;
		print!("{}", world_size);
		for result in results.iter() {
			print!(
				",{}",
				result[i].as_nanos() as f32 / (world_size * world_size) as f32 / run_count as f32
			);
		}
		println!();
	}
}
