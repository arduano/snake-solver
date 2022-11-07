use crate::{
	auto::Path,
	grid_graph::GridGraph,
	snake::{Cell, Direction},
	ui::SnakeWorldViewer,
	Coord, Offset,
};

use super::{basic::BasicSnakeSolver, SnakeSolver};

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
		let food = world.food_coord();

		// Create a random directed graph of edges
		let mut edges = Vec::<Edge>::new();
		for x in (1..world.size() - 1).step_by(2) {
			for y in (1..world.size() - 1).step_by(2) {
				// Is location A valid?
				let a = Coord::new_usize(x, y);
				if check_obstruction(world, a, world.snake_head_coord()) {
					continue;
				}

				for off_x in 0..=1 {
					for off_y in 0..=1 {
						if off_x == 0 && off_y == 0 {
							continue;
						}
						if off_x == 1 && off_y == 1 {
							continue;
						}

						// Unable to reach B
						let t = Coord::new_usize(x + off_x, y + off_y);
						if check_obstruction(world, t, world.snake_head_coord()) {
							continue;
						}

						// Is location A valid?
						let b = Coord::new_usize(x + off_x * 2, y + off_y * 2);
						if check_obstruction(world, b, world.snake_head_coord()) {
							continue;
						}

						// distance from food as percentage
						// let dist = food.get_offset(t).length() as f32 / world.size() as f32;

						edges.push(Edge {
							a,
							b,
							weight: rand::random::<f32>(),
						});
					}
				}
			}
		}

		edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

		// Generate the minimum spanning tree from edges
		let mut visited = Vec::<Coord>::new();
		let mut grid = GridGraph::<bool>::new(world.size() as usize, false);

		// Mark the start point for the spanning tree
		let start = Coord::new_i32(food.x - (food.x % 2) + 1, food.y - (food.y % 2) + 1);
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

					set_grid_edge(&mut grid, pos, vertical);

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

		let mut path = Path::new();
		let mut pos = world.snake_head_coord();
		let mut dir = match world.prev_direction() {
			Some(v) => v,
			None => {
				if grid.get_edge(pos, Direction::Right) == Some(&true) {
					Direction::Up
				} else if grid.get_edge(pos, Direction::Down) == Some(&true) {
					Direction::Right
				} else {
					Direction::Right
				}
			}
		};

		let mut max = world.size() * world.size();
		let mut trailing = false;
		loop {
			match world.get_cell(pos) {
				Some(Cell::Food) => {
					break;
				}
				_ => {}
			}

			let right = dir.rotate_right();
			if grid.get_edge(pos, dir) == None {
				dir = dir.rotate_right();
			} else if grid.get_edge(pos, right) == Some(&true) {
				if grid.get_edge(pos, dir) == Some(&true) {
					dir = dir.rotate_left();
				}
				trailing = true;
			} else if grid.get_edge(pos, dir) == Some(&true) {
				dir = dir.rotate_left();
			} else {
				if trailing {
					dir = dir.rotate_right();
				} else {
					let nx = pos + Offset::from_direction(dir);
					if let Some(&Cell::Snake(_)) = world.get_cell(nx) {
						dir = dir.rotate_right();
					}

					let nx = pos + Offset::from_direction(dir);
					if let Some(&Cell::Snake(_)) = world.get_cell(nx) {
						dir = dir.rotate_left().rotate_left();
					}

					trailing = false;
				}
			}

			pos = pos + Offset::from_direction(dir);
			path.push(dir);

			max -= 1;
			if max == 0 {
				break;
			}
		}

		// Just return the basic path for now
		self.prev_grid = Some(grid);
		return path;
	}

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

fn check_obstruction(world: &crate::snake::SnakeWorld, pos: Coord, head: Coord) -> bool {
	if pos == head {
		return true;
	}

	return match world.get_cell(pos) {
		Some(Cell::Snake(_)) => true,
		_ => false,
	};
}

fn set_grid_edge(grid: &mut GridGraph<bool>, pos: Coord, vertical: bool) {
	grid.set_edge(
		pos,
		match vertical {
			true => Direction::Right,
			false => Direction::Down,
		},
		true,
	);
}
