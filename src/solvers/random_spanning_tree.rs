use crate::{auto::Path, snake::Cell, Coord};

use super::{basic::BasicSnakeSolver, SnakeSolver};

pub struct RandomSpanningTreeSolver {
	pub prev_tree: Option<Vec<Edge>>,
}

impl RandomSpanningTreeSolver {
	pub fn new() -> Self {
		Self { prev_tree: None }
	}
}

pub struct Edge {
	pub a: Coord,
	pub b: Coord,
	pub weight: f32,
}

impl SnakeSolver for RandomSpanningTreeSolver {
	fn get_next_path(&mut self, world: &crate::snake::SnakeWorld) -> Path {
		let mut path = Path::new();

		let mut current_coord = world.snake_head_coord();

		let mut total_vertexes = 0; // number of nodes in the network

		// Create a random directed graph of edges
		let mut edges = Vec::<Edge>::new();
		for x in (0..world.size()).step_by(2) {
			for y in (0..world.size()).step_by(2) {

				// Is location A valid?
				let a = Coord::new_usize(x, y);
				if check_obstruction(world, a) {
					continue;
				}

				let mut has_connection = false;
				for off_x in 0..1 {
					for off_y in 0..1 {
						// Unable to reach B
						if check_obstruction(world, Coord::new_usize(x + off_x, y + off_y)) {
							continue;
						}

						// Is location A valid?
						let b = Coord::new_usize(x + off_x * 2, y + off_y * 2);
						if check_obstruction(world, b) {
							continue;
						}

						edges.push(Edge {
							a, b,
							weight: rand::random(),
						});
						has_connection = true;
						total_vertexes += 1;
					}
				}

				if has_connection {
					total_vertexes += 1;
				}
			}
		}

		edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

		// Generate the minimum spanning tree from edges
		let mut visited = Vec::<Coord>::new();
		let mut tree = Vec::<Edge>::new();

		// Mark the start point for the spanning tree
		visited.push(edges[0].a);

		// has not reached all vertexes
		// & has not ran out of possible connections
		let mut updated = true;
		while updated {
			updated = false;

			let mut i = 0;
			println!("Tree: {}\nVisited: {}\nEdges:{}", tree.len(), visited.len(), edges.len());
			while i < edges.len() {
				let edge = &edges[i];
				let has_a = visited.contains(&edge.a);
				let has_b = visited.contains(&edge.b);

				// If the connection between these two nodes already exists
				// Remove this option as a shorter path has already been taken
				if has_a && has_b {
					// remove edge
					edges.remove(i);
					updated = true;
					println!("  {}", "Edge exists");
					continue;
				}

				// If this connection is new, and plausible from the current tree
				// add this edge
				if has_a || has_b {
					// Remove the edge which is about to be added
					let mut n = edges.remove(i);

					// Push the new node as visited
					if !has_a {
						visited.push(n.a);
					} else if !has_b {
						visited.push(n.b);
					}

					// Swap the edge direction to point existing -> new
					if !has_a && has_b {
						let t = n.a;
						n.a = n.b;
						n.b = t;
					}

					tree.push(n);
					updated = true;
					println!("  {}", "Added edge");
					continue;
				}

				i += 1;
			}
		}

		self.prev_tree = Some(tree);

		// Just return the basic path for now
		return BasicSnakeSolver.get_next_path(world);
	}
}

fn check_obstruction(world: &crate::snake::SnakeWorld, pos: Coord) -> bool {
	return match world.get_cell(pos) {
		Some(Cell::Snake(_)) => true,
		_ => false,
	};
}
