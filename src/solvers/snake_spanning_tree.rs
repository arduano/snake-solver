use crate::{
	auto::Path,
	grid_graph::GridGraph,
	snake::{Cell, Direction},
	Coord, Offset,
};

use super::{basic::BasicSnakeSolver, SnakeSolver};

pub struct SnakeSpanningTreeSolver {
	pub prev_grid: GridGraph<bool>,
	pub snake_grid: GridGraph<bool>,
}

impl SnakeSpanningTreeSolver {
	pub fn new(size: usize) -> Self {
		// Create a random directed graph of edges
		let mut edges = Vec::<Edge>::new();
		for x in (1..size - 2).step_by(2) {
			for y in (1..size - 2).step_by(2) {
				// Is location A valid?
				let a = Coord::new_usize(x, y);

				for off_x in 0..=1 {
					for off_y in 0..=1 {
						if off_x == 0 && off_y == 0 {
							continue;
						}
						if off_x == 1 && off_y == 1 {
							continue;
						}

						// Is location A valid?
						let b = Coord::new_usize(x + off_x * 2, y + off_y * 2);

						edges.push(Edge {
							a,
							b,
							weight: rand::random(),
						});
					}
				}
			}
		}

		edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

		// Generate the minimum spanning tree from edges
		let mut visited = Vec::<Coord>::new();
		let mut grid = GridGraph::<bool>::new(size as usize, false);

		// Mark the start point for the spanning tree
		visited.push(edges[0].a);

		// has not reached all vertexes
		// & has not ran out of possible connections
		let mut updated = true;
		while updated {
			updated = false;

			let mut i = 0;
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
					continue;
				}

				// If this connection is new, and plausible from the current tree
				// add this edge
				if has_a || has_b {
					// Remove the edge which is about to be added
					let mut wall = edges.remove(i);

					// Push the new node as visited
					if !has_a {
						visited.push(wall.a);
					} else if !has_b {
						visited.push(wall.b);
					}

					// Swap the edge direction to point existing -> new
					if !has_a && has_b {
						let t = wall.a;
						wall.a = wall.b;
						wall.b = t;
					}

					let vertical = wall.a.y != wall.b.y;
					let mut pos =
						Coord::new_i32(i32::min(wall.a.x, wall.b.x), i32::min(wall.a.y, wall.b.y));
					if vertical {
						pos.x -= 1;
					} else {
						pos.y -= 1;
					}

					grid.set_edge(
						pos,
						match vertical {
							true => Direction::Right,
							false => Direction::Down,
						},
						true,
					);

					if vertical {
						pos.y += 1;
					} else {
						pos.x += 1;
					}

					grid.set_edge(
						pos,
						match vertical {
							true => Direction::Right,
							false => Direction::Down,
						},
						true,
					);

					updated = true;
					continue;
				}

				i += 1;
			}
		}

		Self {
			prev_grid: grid,
			snake_grid: GridGraph::new(size as usize, false),
		}
	}
}

pub struct Edge {
	pub a: Coord,
	pub b: Coord,
	pub weight: f32,
}

fn get_valid_dirs_from_coord(coord: Coord) -> (Direction, Direction) {
	let twos_coords = [coord.x % 2, coord.y % 2];
	let [clockwise, out] = match twos_coords {
		[0, 0] => [Direction::Right, Direction::Up],
		[1, 0] => [Direction::Down, Direction::Right],
		[1, 1] => [Direction::Left, Direction::Down],
		[0, 1] => [Direction::Up, Direction::Left],
		_ => unreachable!(),
	};

	(clockwise, out)
}

impl SnakeSolver for SnakeSpanningTreeSolver {
	fn get_next_path(&mut self, world: &crate::snake::SnakeWorld) -> Path {
		dbg!(world.food_coord());

		let mut current = world.snake_head_coord();

		let mut path = Path::new();

		loop {
			if let Some(Cell::Food) = world.get_cell(current) {
				break;
			}

			let (clockwise, out) = get_valid_dirs_from_coord(current);

			let next_dir = if self.prev_grid.get_edge(current, clockwise) == Some(&false) {
				clockwise
			} else {
				out
			};

			current = current + Offset::from_direction(next_dir);
			path.push(next_dir);
		}

		self.snake_grid = GridGraph::new(world.size() as usize, false);

		// let set_snake_grid_edge = |pos: Coord, dir: Direction| {

		// };

		let mut current_pos = world.snake_head_coord();

		for dir in world.calculate_snake_path_from_head().iter_directions() {
			let prev = current_pos + Offset::from_direction(dir);
			let dir = dir.opposite();

			let (clockwise, out) = get_valid_dirs_from_coord(prev);

			// It didn't go clockwise therefore it intersected with a wall
			if dir == out {
				self.snake_grid.set_edge(prev, clockwise, true);
				self.snake_grid.set_edge(
					prev + Offset::from_direction(out),
					clockwise,
					true,
				);
			}

			current_pos = prev;
		}

		path
	}
}
