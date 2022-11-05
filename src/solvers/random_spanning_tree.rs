use crate::{
	auto::Path,
	snake::{Cell, Direction},
	Offset, Coord,
};

use rand::Rng;

use super::SnakeSolver;

/// Generates a path that zigzags until the food, then when it reaches the bottom it goes up along the left.
/// This only works in evenly-sized worlds.
pub struct BasicSnakeSolver;

struct Edge {
	a: Coord,
	b: Coord,
	weight: f32
}

impl SnakeSolver for BasicSnakeSolver {
	fn get_next_path(world: &crate::snake::SnakeWorld) -> Path {
		let mut path = Path::new();

		let mut current_coord = world.snake_head_coord();

		// Create a random directed graph of edges
		let mut edges = Vec::<Edge>::new();
		for x in (0..world.size()).step_by(2) {
			for y in (0..world.size()).step_by(2) {
				let a = Coord::new_usize(x, y);

				// Skip invalid locations
				if (check_obstruction(world, a)) {
					continue;
				}

				for offX in 0..1 {
					for offY in 0..1 {
						let tween = Coord::new_usize(x + offX, y + offY);
						// Skip invalid path
						if (check_obstruction(world, tween)) {
							continue;
						}

						let b = Coord::new_usize(x + offX*2, y + offY*2);

						// Skip invalid locations
						if (check_obstruction(world, b)) {
							continue;
						}

						edges.push(Edge {
							a, b,
							weight: rand::random()
						})
					}
				}
			}
		}

		return path;
	}
}


fn check_obstruction(world: &crate::snake::SnakeWorld, pos: Coord) -> bool {
	return match world.get_cell(pos) {
		Some(Cell::Snake(_)) => true,
		_ => false
	};
}