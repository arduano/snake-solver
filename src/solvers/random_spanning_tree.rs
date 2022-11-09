extern crate rand;

use crate::{auto::Path, direction::Direction, grid_graph::GridGraph, ui::SnakeWorldViewer, Coord};

use super::{utils::build_path_from_collision_grid, SnakeSolver};

pub struct RandomSpanningTreeSolver {
	pub prev_grid: Option<GridGraph<bool>>,
}

impl RandomSpanningTreeSolver {
	pub fn new() -> Self {
		Self { prev_grid: None }
	}
}

pub struct Edge {
	pub a: Coord,
	pub b: Coord,
	pub weight: f32,
}

impl SnakeSolver for RandomSpanningTreeSolver {
	fn get_next_path(&mut self, world: &crate::snake::SnakeWorld) -> Path {
		// Generate the graph over every second grid square with minimum weights
		// Then convert those edges into a MST
		// And convert that MST into a collision space
		let grid = match self.prev_grid.take() {
			None => generate_grid_network(world, generate_edges(world)),
			Some(grid) => grid,
		};

		let path = build_path_from_collision_grid(&grid, world);
		self.prev_grid = Some(grid);

		return path;
	}

	// UI code for drawing the collision grid
	fn decorate_widget<'a>(&'a self, widget: SnakeWorldViewer<'a>) -> SnakeWorldViewer<'a> {
		if let Some(prev_grid) = &self.prev_grid {
			widget.with_bools_edges_grid_overlay(
				prev_grid,
				eframe::egui::Color32::from_rgb(0, 0, 255),
			)
		} else {
			widget
		}
	}
}

// Generate all of the random edges for a given world
fn generate_edges(world: &crate::snake::SnakeWorld) -> Vec<Edge> {
	let _food = world.food_coord();

	// Create a random directed graph of edges
	// Connecting every second square to it's direct neighbour (not diagonally)
	let mut edges = Vec::<Edge>::new();
	for x in (1..world.size() - 1).step_by(2) {
		for y in (1..world.size() - 1).step_by(2) {
			let a = Coord::new(x, y);
			for (off_x, off_y) in [(0, 1), (1, 0)] {
				let b = Coord::new(x + off_x * 2, y + off_y * 2);

				edges.push(Edge {
					a,
					b,
					weight: rand::random::<f32>(),
				});
			}
		}
	}

	// Add the missing edges to the bottom right node
	// Because the above loop omitted them for simplicity
	let size = world.size();
	edges.push(Edge {
		a: Coord::new(size - 1, size - 3),
		b: Coord::new(size - 1, size - 1),
		weight: rand::random::<f32>(),
	});
	edges.push(Edge {
		a: Coord::new(size - 3, size - 1),
		b: Coord::new(size - 1, size - 1),
		weight: rand::random::<f32>(),
	});

	// Sort the edges by weight for the later MST calculations
	edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

	return edges;
}

fn generate_grid_network(
	world: &crate::snake::SnakeWorld,
	mut edges: Vec<Edge>,
) -> GridGraph<bool> {
	let food = world.food_coord();

	// Generate the minimum spanning tree from edges
	//   Instead of generating the tree structure completely
	//   It is only partially generated as it will be converted into a collision GridGraph anyway
	//   Thus we only store the necessary information to continue building a valid MST
	//     rather than enough to store it
	let mut visited = Vec::<Coord>::new();
	let mut grid = GridGraph::<bool>::new(world.size() as usize, false);

	// Mark the start point for the spanning tree
	let start = Coord::new(food.x - (food.x % 2) + 1, food.y - (food.y % 2) + 1);
	visited.push(start);

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
				//   as the graph is non-directed
				if !has_a && has_b {
					let t = wall.a;
					wall.a = wall.b;
					wall.b = t;
				}

				// Convert from the graph coordinates to grid coordinates
				let vertical = wall.a.y != wall.b.y;
				let mut pos =
					Coord::new(i32::min(wall.a.x, wall.b.x), i32::min(wall.a.y, wall.b.y));
				if vertical {
					pos.x -= 1;
				} else {
					pos.y -= 1;
				}
				// Update the collision grid to handle this edge
				set_grid_edge(&mut grid, pos, vertical);

				// As each graph node spans two grid points
				//  The second half of the edge needs to be added
				if vertical {
					pos.y += 1;
				} else {
					pos.x += 1;
				}
				set_grid_edge(&mut grid, pos, vertical);

				updated = true;
				continue;
			}

			i += 1;
		}
	}

	return grid;
}

fn set_grid_edge(grid: &mut GridGraph<bool>, pos: Coord, vertical: bool) {
	grid.try_set_edge(
		pos,
		match vertical {
			true => Direction::Right,
			false => Direction::Down,
		},
		true,
	);
}
