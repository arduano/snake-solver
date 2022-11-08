use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use snake_solver::{
	auto::{AutoPlayerState, AutoSnakePlayer},
	snake::SnakeResult,
	solvers::{
		random_spanning_tree::RandomSpanningTreeSolver,
		snake_spanning_tree::SnakeSpanningTreeSolver, SnakeSolver,
	},
	ui::SnakeWorldViewer,
};

use eframe::egui::{self};

fn make_solver() -> impl SnakeSolver {
	SnakeSpanningTreeSolver::new(None)
}

fn run_benches(sizes: &[usize]) -> Vec<u64> {
	let mut results = Vec::new();
	for size in sizes {
		let solver = make_solver();
		let mut world = AutoSnakePlayer::new(*size, solver);
		let mut total_time = 0;
		loop {
			let result = world.step();

			match result {
				SnakeResult::Finished => break,
				SnakeResult::Killed => {
					panic!("Killed");
				}
				SnakeResult::Stepped => {
					total_time += 1;
				}
			}
		}
		results.push(total_time);
	}
	results
}

fn main() {
	let sizes = [10, 20, 40, 60, 80];
	let runs_per_size = 100;

	let all_results = (0..runs_per_size)
		.into_par_iter()
		.map(|_| run_benches(&sizes))
		.progress_count(runs_per_size)
		.collect::<Vec<_>>();

	for i in 0..sizes.len() {
		// Print the min, avg and max
		let mut results = all_results.iter().map(|run| run[i]).collect::<Vec<_>>();
		results.sort();
		let min = results[0];
		let max = results[results.len() - 1];
		let avg = results.iter().sum::<u64>() as f64 / results.len() as f64;
		println!("Size: {}, Min: {}, Avg: {}, Max: {}", sizes[i], min, avg, max);
	}
}
