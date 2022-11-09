use std::collections::VecDeque;

use rand::Rng;

use crate::{
	array2d::Array2D,
	auto::Path,
	grid_graph::GridGraph,
	snake::{Cell, Direction, SnakeWorld},
	ui::SnakeWorldViewer,
	Coord, Offset,
};

use super::{
	utils::{build_path_from_collision_grid, get_valid_dirs_from_coord},
	SnakeSolver,
};

pub struct SnakeSpanningTreeSolver {
	pub prev_pathfinding_grid: Option<Array2D<u32>>,
	pub snake_grid: Option<GridGraph<bool>>,
	iteratively_repath_after: Option<usize>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SpanTreeEdgeType {
	Free,
	Wall,
	CoveredByCurrentSnake,
	CoveredByFutureSnake,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridStepKind {
	Clockwise,
	Out,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SnakePathfindResult {
	Success,
	SuccessWithPathOverride,
}

impl SpanTreeEdgeType {
	fn is_free(&self) -> bool {
		use SpanTreeEdgeType::*;
		matches!(self, Free | CoveredByCurrentSnake | CoveredByFutureSnake)
	}

	fn is_taken(&self) -> bool {
		!self.is_free()
	}
}

impl SnakeSpanningTreeSolver {
	pub fn new(iteratively_repath_after: Option<usize>) -> Self {
		Self {
			snake_grid: None,
			prev_pathfinding_grid: None,
			iteratively_repath_after,
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
	pathfinding_grid: Array2D<u32>,
}

impl<'a> SnakeCalculator<'a> {
	fn new(snake_world: &'a SnakeWorld) -> Self {
		Self {
			snake_spanning_tree: GridGraph::new(
				snake_world.size() as usize / 2,
				SpanTreeEdgeType::Free,
			),
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

			let [clockwise, out] = get_valid_dirs_from_coord(prev);

			let mut mark_edge = |coord: Coord, dir: Direction, kind: SpanTreeEdgeType| {
				let (meta_coord, new_dir) = Self::calculate_inner_tree_coord(coord, dir);

				self.snake_spanning_tree
					.try_set_edge(meta_coord, new_dir, kind);
			};

			// It didn't go clockwise therefore it intersected with a wall
			if dir == out {
				mark_edge(prev, clockwise, SpanTreeEdgeType::Wall);
			}

			// It didn't go clockwise therefore it intersected with a wall
			if dir == clockwise {
				mark_edge(prev, clockwise, SpanTreeEdgeType::CoveredByCurrentSnake);
			}

			current_pos = prev;
		}
	}

	fn fill_pathfinding_grid(&mut self) {
		let starting_coord = self.snake_world.food_coord();

		let mut queue = VecDeque::new();
		queue.push_back((starting_coord, 1));

		while let Some((coord, dist)) = queue.pop_front() {
			self.pathfinding_grid.set(coord, dist);

			let [clockwise, out] = get_valid_dirs_from_coord(coord);
			let items = [
				(clockwise.opposite(), GridStepKind::Out),
				(out.opposite(), GridStepKind::Clockwise),
			];

			for (dir, kind) in items.into_iter() {
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

				if current_val != 0 {
					continue;
				}

				match kind {
					GridStepKind::Clockwise => {
						if !self.can_walk_clockwise_from(next) {
							continue;
						}
					}
					GridStepKind::Out => {
						if !self.can_walk_out_from(next) {
							continue;
						}
					}
				}

				self.pathfinding_grid.set(next, u32::MAX);
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

	fn can_walk_clockwise_from(&self, coord: Coord) -> bool {
		let [clockwise, _] = get_valid_dirs_from_coord(coord);
		let (coord, dir) = Self::calculate_inner_tree_coord(coord, clockwise);

		let connecting_edge = self.snake_spanning_tree.get_edge(coord, dir);
		let result = match connecting_edge {
			Some(edge) => edge.is_free(),
			_ => true,
		};

		result
	}

	fn pathfind_and_grow(&mut self) -> Result<SnakePathfindResult, ()> {
		let mut current = self.snake_world.snake_head_coord();

		loop {
			let current_value = *self.pathfinding_grid.get(current).unwrap();
			if current_value == 1 {
				break;
			}

			let [clockwise, out] = get_valid_dirs_from_coord(current);

			let get_value_at = |dir: Direction| {
				let coord = current + Offset::from_direction(dir);
				self.pathfinding_grid
					.get(coord)
					.and_then(|&v| if v == 0 { None } else { Some(v) })
			};

			let clockwise_value = get_value_at(clockwise);
			let out_value = get_value_at(out);

			if clockwise_value == None && out_value == None {
				println!("Reached a dead end when pathfinding");
				return Err(());
			}

			let next_dir = if clockwise_value == None {
				GridStepKind::Out
			} else if out_value == None {
				GridStepKind::Clockwise
			} else if clockwise_value < out_value {
				GridStepKind::Clockwise
			} else if self.can_walk_out_from(current) {
				GridStepKind::Out
			} else {
				GridStepKind::Clockwise
			};

			if next_dir == GridStepKind::Out {
				let (coord, dir) = Self::calculate_following_out_edge(current);
				self.snake_spanning_tree
					.try_set_edge(coord, dir, SpanTreeEdgeType::Wall);
			}

			if next_dir == GridStepKind::Clockwise {
				let (coord, dir) = Self::calculate_following_out_edge(current);
				self.snake_spanning_tree.try_set_edge(
					coord,
					dir,
					SpanTreeEdgeType::CoveredByFutureSnake,
				);
			}

			let next_dir = match next_dir {
				GridStepKind::Clockwise => clockwise,
				GridStepKind::Out => out,
			};

			current = current + Offset::from_direction(next_dir);
		}

		let mut allow_covered = false;

		let mut seeded_covered_edge = false;

		// Seed the tree from here
		'iter: loop {
			for x in 0..self.snake_spanning_tree.size() {
				for y in 0..self.snake_spanning_tree.size() {
					let coord = Coord::new_usize(x, y);
					if !self.is_spanning_tree_coord_taken(coord) {
						continue;
					}

					for dir in Direction::each() {
						if let Some(&edge) = self.snake_spanning_tree.get_edge(coord, dir) {
							use SpanTreeEdgeType::*;
							let free =
								edge == Free || (allow_covered && edge == CoveredByFutureSnake);

							if free {
								let seed_coord = coord + Offset::from_direction(dir);
								if !self.is_spanning_tree_coord_taken(seed_coord) {
									self.snake_spanning_tree.set_edge(
										coord,
										dir,
										SpanTreeEdgeType::Wall,
									);

									self.seed_tree_from(seed_coord);

									if allow_covered {
										seeded_covered_edge = true;
									}

									continue 'iter;
								}
							}
						}
					}
				}
			}

			if !allow_covered {
				allow_covered = true;
				continue;
			} else {
				break;
			}
		}

		if seeded_covered_edge {
			Ok(SnakePathfindResult::SuccessWithPathOverride)
		} else {
			Ok(SnakePathfindResult::Success)
		}
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
		let [_, out] = get_valid_dirs_from_coord(coord);
		(meta_coord, out)
	}

	fn seed_tree_from(&mut self, coord: Coord) {
		let mut stack = VecDeque::new();
		let mut last_coord = coord;

		let mut possible_dirs = Vec::with_capacity(4);
		stack.push_back(last_coord);

		loop {
			let current_coord = last_coord;

			for dir in Direction::each() {
				let next_coord = current_coord + Offset::from_direction(dir);
				if !self.snake_spanning_tree.is_in_bounds(next_coord) {
					continue;
				}

				if !self.is_spanning_tree_coord_taken(next_coord) {
					possible_dirs.push(dir);
				}
			}

			if possible_dirs.len() == 0 {
				last_coord = match stack.pop_back() {
					Some(coord) => coord,
					None => break,
				};
				continue;
			}

			let dir = possible_dirs[rand::thread_rng().gen_range(0..possible_dirs.len())];
			// let dir = possible_dirs[0];
			possible_dirs.clear();

			let next_coord = current_coord + Offset::from_direction(dir);
			stack.push_back(next_coord);
			last_coord = next_coord;

			self.snake_spanning_tree
				.set_edge(current_coord, dir, SpanTreeEdgeType::Wall);
		}
	}

	fn is_spanning_tree_coord_taken(&self, coord: Coord) -> bool {
		for dir in Direction::each() {
			let edge = self.snake_spanning_tree.get_edge(coord, dir);
			if let Some(edge) = edge {
				if edge.is_taken() {
					return true;
				}
			}
		}

		false
	}

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

impl SnakeSolver for SnakeSpanningTreeSolver {
	fn get_next_path(&mut self, world: &SnakeWorld) -> Path {
		let mut calculator = SnakeCalculator::new(world);
		calculator.trace_current_snake_and_mark_edges();
		// calculator.set_test_edge();
		calculator.fill_pathfinding_grid();

		let result = calculator.pathfind_and_grow();

		let collision_grid = calculator.build_collision_grid_from_spanning_tree();

		let path = match result {
			Err(()) => {
				println!("Pathfinding failed, returning a killing path");

				let mut path = Path::new();
				path.push(
					world
						.calculate_snake_path_from_head()
						.iter_directions()
						.nth(0)
						.unwrap(),
				);
				path
			}
			Ok(kind) => {
				let path = build_path_from_collision_grid(&collision_grid, &world);

				if kind == SnakePathfindResult::Success {
					path
				} else if let Some(count) = self.iteratively_repath_after {
					let mut clipped_path = Path::new();

					for dir in path.iter_directions().take(count) {
						clipped_path.push(dir);
					}

					clipped_path
				} else {
					path
				}
			}
		};

		self.snake_grid = Some(collision_grid);
		self.prev_pathfinding_grid = Some(calculator.pathfinding_grid);

		path
	}

	fn decorate_widget<'a>(&'a self, mut widget: SnakeWorldViewer<'a>) -> SnakeWorldViewer<'a> {
		if let Some(snake_grid) = &self.snake_grid {
			widget = widget.with_bools_edges_grid_overlay(
				snake_grid,
				eframe::egui::Color32::from_rgb(0, 255, 255),
			);
		}

		if let Some(prev_pathfinding_grid) = &self.prev_pathfinding_grid {
			widget = widget.with_pathfinding_grid_overlay(prev_pathfinding_grid);
		}

		widget
	}
}
