use crate::{
	auto::Path,
	snake::{Cell, Direction},
	Coord, Offset,
};

use rand::Rng;

use super::SnakeSolver;

/// Generates a path that zigzags until the food, then when it reaches the bottom it goes up along the left.
/// This only works in evenly-sized worlds.
pub struct BasicSnakeSolver;

struct Edge {
	a: Coord,
	b: Coord,
	weight: f32,
}

impl SnakeSolver for BasicSnakeSolver {
	fn get_next_path(world: &crate::snake::SnakeWorld) -> Path {
		let mut path = Path::new();

		let mut current_coord = world.snake_head_coord();

		let mut totalVertexes = 0; // number of nodes in the network

		// Create a random directed graph of edges
		let mut edges = Vec::<Edge>::new();
		for x in (0..world.size()).step_by(2) {
			for y in (0..world.size()).step_by(2) {
				let a = Coord::new_usize(x, y);

				// Skip invalid locations
				if (check_obstruction(world, a)) {
					continue;
				}

				let mut hasConnection = false;
				for offX in 0..1 {
					for offY in 0..1 {
						let tween = Coord::new_usize(x + offX, y + offY);
						// Skip invalid path
						if (check_obstruction(world, tween)) {
							continue;
						}

						let b = Coord::new_usize(x + offX * 2, y + offY * 2);

						// Skip invalid locations
						if (check_obstruction(world, b)) {
							continue;
						}

						edges.push(Edge {
							a,
							b,
							weight: rand::random(),
						})
						hasConnection = true;
						totalVertexes += 1;
					}
				}

				if (hasConnection) {
					totalVertexes += 1;
				}
			}
		}

		edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());


		// Generate the minimum spanning tree from edges
		let mut visited = Vec::<Coord>::new();
		visited.push(edges[0].a);

		let tree = Vec::<Edge>::new();

		// has not reached all vertexs
		// & has not ran out of possible connections
		while (tree.len()+1 < totalVertexes && edges.len() > 0) {
			let mut i = 0;
			while i < edges.len() {
				let edge = edges[i];
				let hasA = visited.contains(&edge.a);
				let hasB = visited.contains(&edge.b);

				// If the connection between these two nodes already exists
				// Remove this option as a shorter path has already been taken
				if (hasA && hasB) {
					// remove edge
					edges.remove(i);
					continue;
				}

				// If this connection is new, and plauible from the current tree
				// add this edge
				if (hasA || hasB) {
					// Swap the edges so it goes from existing to new
					if (!hasA && hasB) {
						let t = edge.a;
						edge.a = edge.b;
						edge.b = t;
					}

					tree.push(edge);
					edges.remove(i);
					continue;
				}

				i += 1;
			}
		}

		return path;
	}
}

fn check_obstruction(world: &crate::snake::SnakeWorld, pos: Coord) -> bool {
	return match world.get_cell(pos) {
		Some(Cell::Snake(_)) => true,
		_ => false,
	};
}
