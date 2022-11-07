use std::collections::VecDeque;

use crate::{
	array2d::Array2D,
	auto::Path,
	grid_graph::GridGraph,
	snake::{Cell, Direction, SnakeWorld},
	ui::SnakeWorldViewer,
	Coord, Offset,
};

use super::{basic::BasicSnakeSolver, SnakeSolver};

pub struct SnakeSpanningTreeSolver {
	pub prev_grid: GridGraph<bool>,
	pub prev_pathfinding_grid: Array2D<u32>,
	pub snake_grid: GridGraph<bool>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SpanTreeEdgeType {
	Free,
	Wall,
	CoveredByFutureSnake,
	CoveredBySnake,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridStepKind {
	Clockwise,
	Out,
}

impl SpanTreeEdgeType {
	fn is_free(&self) -> bool {
		matches!(
			self,
			SpanTreeEdgeType::Free | SpanTreeEdgeType::CoveredByFutureSnake
		)
	}

	fn is_taken(&self) -> bool {
		!self.is_free()
	}
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
			prev_pathfinding_grid: Array2D::new(size as usize, 0),
		}
	}
}

pub struct Edge {
	pub a: Coord,
	pub b: Coord,
	pub weight: f32,
}

struct SnakeCalculator<'a> {
	snake_spanning_tree: GridGraph<SpanTreeEdgeType>,
	snake_world: &'a SnakeWorld,
	current_snake_coord: Coord,
	pathfinding_grid: Array2D<u32>,
}

impl<'a> SnakeCalculator<'a> {
	fn new(snake_world: &'a SnakeWorld) -> Self {
		Self {
			snake_spanning_tree: GridGraph::new(
				snake_world.size() as usize / 2,
				SpanTreeEdgeType::Free,
			),
			current_snake_coord: snake_world.snake_head_coord(),
			snake_world,
			pathfinding_grid: Array2D::new(snake_world.size() as usize, 0),
		}
	}

	fn trace_current_snake_and_mark_edges(&mut self) {
		let mut current_pos = self.snake_world.snake_head_coord();

		for dir in self
			.snake_world
			.calculate_snake_path_from_head()
			.iter_directions()
		{
			let prev = current_pos + Offset::from_direction(dir);
			let dir = dir.opposite();

			let (clockwise, out) = get_valid_dirs_from_coord(prev);

			let mut mark_edge = |coord: Coord, dir: Direction, kind: SpanTreeEdgeType| {
				let meta_coord = coord.map_values(|v| (v + 1) / 2 - 1);

				let meta_coord = match dir {
					Direction::Left | Direction::Up => meta_coord,
					Direction::Right | Direction::Down => meta_coord + Offset::new(1, 1),
				};

				let new_dir = match dir {
					Direction::Left | Direction::Right => dir.rotate_left(),
					Direction::Up | Direction::Down => dir.rotate_right(),
				};

				self.snake_spanning_tree
					.try_set_edge(meta_coord, new_dir, kind);
			};

			// It didn't go clockwise therefore it intersected with a wall
			if dir == out {
				mark_edge(prev, clockwise, SpanTreeEdgeType::Wall);
			}

			if dir == clockwise {
				mark_edge(prev, clockwise, SpanTreeEdgeType::CoveredBySnake);
			}

			current_pos = prev;
		}
	}

	fn set_test_edge(&mut self) {
		let (coord, dir) = Self::calculate_following_out_edge(Coord::new_i32(1, 1));
		self.snake_spanning_tree
			.try_set_edge(coord, dir, SpanTreeEdgeType::Wall);

		let (coord, dir) = Self::calculate_following_out_edge(Coord::new_i32(1, 5));
		self.snake_spanning_tree
			.try_set_edge(coord, dir, SpanTreeEdgeType::Wall);

		dbg!(self.can_walk_out_from(Coord::new_i32(1, 1)));
	}

	fn fill_pathfinding_grid(&mut self) {
		let starting_coord = self.snake_world.food_coord();

		let mut queue = VecDeque::new();
		queue.push_back((starting_coord, 1));

		while let Some((coord, dist)) = queue.pop_front() {
			self.pathfinding_grid.set(coord, dist);

			let (clockwise, out) = get_valid_dirs_from_coord(coord);
			let reversed = [clockwise, out].into_iter().map(|dir| dir.opposite());

			for dir in reversed {
				let next = coord + Offset::from_direction(dir);

				let Some(cell) = self.snake_world.get_cell(next) else {
					continue;
				};

				if let Cell::Snake(_) = cell {
					continue;
				}

				let Some(&current_val) = self.pathfinding_grid.get(next) else {
					continue;
				};

				if current_val != 0 || current_val <= dist + 1 {
					continue;
				}

				queue.push_back((next, dist + 1));
			}
		}
	}

	/// Check if the snake can pathfind from the current cell outwards. It can can't do it if the next
	/// node is taken and the connecting edge isn't, which implies that the new connection may cause a loop.
	fn can_walk_out_from(&self, coord: Coord) -> bool {
		let (coord, dir) = Self::calculate_following_out_edge(coord);
		let connecting_edge = self.snake_spanning_tree.get_edge(coord, dir);
		let connecting_edge_taken = match connecting_edge {
			Some(edge) => edge.is_taken(),
			_ => false,
		};

		let next_cell = coord + Offset::from_direction(dir);
		let other_dirs = [dir, dir.rotate_left(), dir.rotate_right()];
		let mut other_edges = other_dirs
			.iter()
			.map(|dir| self.snake_spanning_tree.get_edge(next_cell, *dir));
		let next_cell_taken = other_edges.any(|edge| match edge {
			None => false,
			Some(edge) => edge.is_taken(),
		});

		// If the next cell isn't taken, it's ok
		if !next_cell_taken {
			return true;
		}

		// If the next cell is taken but the connecting edge is also taken, it's ok
		if connecting_edge_taken {
			return true;
		}

		return false;
	}

	fn calculate_inner_tree_coord(coord: Coord, dir: Direction) -> (Coord, Direction) {
		let meta_coord = coord.map_values(|v| (v + 1) / 2 - 1);

		let meta_coord = match dir {
			Direction::Left | Direction::Up => meta_coord,
			Direction::Right | Direction::Down => meta_coord + Offset::new(1, 1),
		};

		let new_dir = match dir {
			Direction::Left | Direction::Right => dir.rotate_left(),
			Direction::Up | Direction::Down => dir.rotate_right(),
		};

		(meta_coord, new_dir)
	}

	fn calculate_following_out_edge(coord: Coord) -> (Coord, Direction) {
		let meta_coord = coord.map_values(|v| v / 2);
		let (_, out) = get_valid_dirs_from_coord(coord);
		(meta_coord, out)
	}

	// fn can_go_out_at(coord: Coord) -> bool {
	// 	let meta_coord = coord.map_values(|v| (v + 1) / 2 - 1);

	// 	let meta_coord = match dir {
	// 		Direction::Left | Direction::Up => meta_coord,
	// 		Direction::Right | Direction::Down => meta_coord + Offset::new(1, 1),
	// 	};

	// 	let new_dir = match dir {
	// 		Direction::Left | Direction::Right => dir.rotate_left(),
	// 		Direction::Up | Direction::Down => dir.rotate_right(),
	// 	};

	// 	(meta_coord, new_dir)
	// }

	fn get_wall(&self, coord: Coord, dir: Direction) -> SpanTreeEdgeType {
		let (meta_coord, new_dir) = Self::calculate_inner_tree_coord(coord, dir);
		*self
			.snake_spanning_tree
			.get_edge(meta_coord, new_dir)
			.unwrap_or(&SpanTreeEdgeType::Free)
	}

	fn set_wall(&mut self, coord: Coord, dir: Direction, value: SpanTreeEdgeType) {
		let (meta_coord, new_dir) = Self::calculate_inner_tree_coord(coord, dir);
		self.snake_spanning_tree
			.set_edge(meta_coord, new_dir, value);
	}

	// fn is_wall_blocking(&mut self, coord: Coord, dir: Direction) {
	// 	let (meta_coord, new_dir) = Self::calculate_inner_tree_coord(coord, dir);
	// 	self.snake_spanning_tree
	// 		.set_edge(meta_coord, new_dir, value);
	// }

	// fn can_move_clockwise(&mut self, coord: Coord, dir: Direction) {
	// 	let (meta_coord, new_dir) = Self::calculate_inner_tree_coord(coord, dir);
	// 	self.snake_spanning_tree
	// 		.set_edge(meta_coord, new_dir, value);
	// }

	// fn can_move_from(&mut self, coord: Coord, dir: Direction) {
	// 	let (meta_coord, new_dir) = Self::calculate_inner_tree_coord(coord, dir);
	// 	self.snake_spanning_tree
	// 		.set_edge(meta_coord, new_dir, value);
	// }

	// fn can_move(&mut self, dir: Direction) {
	// 	self.can_move_from(self.current_snake_coord, dir);
	// }

	fn build_collision_grid_from_spanning_tree(&self) -> GridGraph<bool> {
		let mut snake_grid = GridGraph::new(self.snake_world.size() as usize, false);

		let mut set_snake_grid_edge = |pos: Coord, dir: Direction| {
			let pos = pos.map_values(|val| val * 2 + 1);

			let left = dir.rotate_left();

			let start_offset = match dir {
				Direction::Right => Offset::new(0, 0),
				Direction::Up => Offset::new(0, -1),
				Direction::Down => Offset::new(-1, 0),
				Direction::Left => Offset::new(-1, -1),
			};

			let pos = pos + start_offset;
			snake_grid.set_edge(pos, left, true);
			let pos = pos + Offset::from_direction(dir);
			snake_grid.set_edge(pos, left, true);
		};

		for x in 0..self.snake_spanning_tree.size() {
			for y in 0..self.snake_spanning_tree.size() {
				for dir in Direction::each() {
					if self
						.snake_spanning_tree
						.get_edge(Coord::new_usize(x, y), dir)
						== Some(&SpanTreeEdgeType::Wall)
					{
						set_snake_grid_edge(Coord::new_usize(x, y), dir);
					}
				}
			}
		}

		snake_grid
	}
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
	fn get_next_path(&mut self, world: &SnakeWorld) -> Path {
		let mut calculator = SnakeCalculator::new(world);
		calculator.trace_current_snake_and_mark_edges();
		calculator.fill_pathfinding_grid();
		// calculator.set_test_edge();

		self.snake_grid = calculator.build_collision_grid_from_spanning_tree();
		self.prev_pathfinding_grid = calculator.pathfinding_grid;

		let path = build_path_from_collision_grid(&self.prev_grid, &world);
		path
	}

	fn decorate_widget<'a>(&'a self, widget: SnakeWorldViewer<'a>) -> SnakeWorldViewer<'a> {
		widget
			.with_bools_edges_grid_overlay(
				&self.snake_grid,
				eframe::egui::Color32::from_rgb(0, 255, 255),
			)
			.with_bools_edges_grid_overlay(
				&self.prev_grid,
				eframe::egui::Color32::from_rgb(0, 0, 255),
			)
	}
}

fn build_path_from_collision_grid(grid: &GridGraph<bool>, world: &SnakeWorld) -> Path {
	let mut current = world.snake_head_coord();

	let mut path = Path::new();
	loop {
		if let Some(Cell::Food) = world.get_cell(current) {
			break;
		}

		let (clockwise, out) = get_valid_dirs_from_coord(current);

		let next_dir = if grid.get_edge(current, clockwise) == Some(&false) {
			clockwise
		} else {
			out
		};

		current = current + Offset::from_direction(next_dir);
		path.push(next_dir);
	}

	path
}
