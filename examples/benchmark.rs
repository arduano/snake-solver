use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use snake_solver::{
	auto::AutoSnakePlayer,
	snake::SnakeResult,
	solvers::{basic::BasicSnakeSolver, snake_spanning_tree::SnakeSpanningTreeSolver, SnakeSolver, random_spanning_tree::RandomSpanningTreeSolver},
};

fn run_benches<SS: SnakeSolver>(sizes: &[usize], make_solver: impl Fn() -> SS) -> Vec<u64> {
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

fn run_all_benches<SS: SnakeSolver>(name: &str, make_solver: impl Send + Sync + Fn() -> SS) {
	let sizes = [10, 20, 40, 60, 80];
	let runs_per_size = 100;

	println!("{}", name);

	let all_results = (0..runs_per_size)
		.into_par_iter()
		.map(|_| run_benches(&sizes, &make_solver))
		.progress_count(runs_per_size)
		.collect::<Vec<_>>();

	for i in 0..sizes.len() {
		// Print the min, avg and max
		let mut results = all_results.iter().map(|run| run[i]).collect::<Vec<_>>();
		results.sort();
		let min = results[0];
		let max = results[results.len() - 1];
		let avg = results.iter().sum::<u64>() as f64 / results.len() as f64;
		println!(
			"Size: {}, Min: {}, Avg: {}, Max: {}",
			sizes[i], min, avg, max
		);
	}
	println!("");
}

fn main() {
	println!();
	run_all_benches("Brute force:", || BasicSnakeSolver);
	run_all_benches("Random hamiltonian:", || RandomSpanningTreeSolver::new());
	println!();
	run_all_benches("Pathfinding hamiltonian:", || {
		SnakeSpanningTreeSolver::new(None)
	});
	println!();
	run_all_benches("Pathfinding hamiltonian with repathing:", || {
		SnakeSpanningTreeSolver::new(Some(10))
	});
}
