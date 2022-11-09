use std::collections::VecDeque;

use rand::Rng;

use crate::{
	direction::Direction,
	grid_graph::GridGraph,
	path::Path,
	snake::{Cell, SnakeWorld},
	solvers::utils::get_valid_dirs_from_coord,
	Coord, Offset,
};

use super::coordinates::{calculate_following_out_edge, calculate_inner_tree_coord};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpanTreeEdgeType {
	Free,
	Wall,
	CoveredByCurrentSnake,
	CoveredByFutureSnake,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SnakeGrowResult {
	Success,
	SuccessWithPathOverride,
}

impl SpanTreeEdgeType {
	pub fn is_free(&self) -> bool {
		use SpanTreeEdgeType::*;
		matches!(self, Free | CoveredByCurrentSnake | CoveredByFutureSnake)
	}

	pub fn is_taken(&self) -> bool {
		!self.is_free()
	}
}

pub struct SpanningTree {
	graph: GridGraph<SpanTreeEdgeType>,
}

impl SpanningTree {
	pub fn new(world_size: usize) -> Self {
		let graph = GridGraph::new(world_size as usize / 2, SpanTreeEdgeType::Free);
		Self { graph }
	}

	/// Follow a snake's path from the head, cell by cell, and mark which edges
	/// the snake stencils out and which edges it covers.
	pub fn trace_current_snake_and_mark_edges(&mut self, world: &SnakeWorld) {
		let mut current_pos = world.snake_head_coord();

		// We iterate over all of the snake's directions from the head
		for dir in world.calculate_snake_path_from_head().iter_directions() {
			let prev = current_pos.go_towards(dir);
			let dir = dir.opposite();

			let [clockwise, out] = get_valid_dirs_from_coord(prev);

			let mut mark_edge = |coord: Coord, dir: Direction, kind: SpanTreeEdgeType| {
				let (meta_coord, new_dir) = calculate_inner_tree_coord(coord, dir);
				self.try_set_edge(meta_coord, new_dir, kind);
			};

			// We check the taken direction below and compare it to the clockwise rules.

			if dir == out {
				// It didn't go clockwise, therefore it intersected with a wall
				mark_edge(prev, clockwise, SpanTreeEdgeType::Wall);
			}

			if dir == clockwise {
				// It went clockwise, therefore it there shouldn't be a wall there
				mark_edge(prev, clockwise, SpanTreeEdgeType::CoveredByCurrentSnake);
			}

			current_pos = prev;
		}
	}

	/// Grow the spanning tree across the entire grid. First attempt to not grow over edges
	/// that are `CoveredByCurrentSnake`, and if there's still holes remaining after then
	/// grow over `CoveredByFutureSnake` edges. `SnakeGrowResult` reflects whether
	/// the spanning tree was grown over any `CoveredByFutureSnake` edges.
	pub fn grow_spanning_tree(&mut self) -> SnakeGrowResult {
		let mut allow_covered = false;

		let mut seeded_covered_edge = false;

		'iter: loop {
			// We loop over each coodinate and direction to find a taken coordinate with a neighbouring untaken coordinate
			for coord in self.iter_all_coords() {
				if !self.is_tree_node_taken(coord) {
					continue;
				}

				for dir in Direction::each() {
					if let Some(&edge) = self.get_edge(coord, dir) {
						use SpanTreeEdgeType::*;
						// The edge is free if it's free or it's covered and we're overriding covered edges.
						let free = edge == Free || (allow_covered && edge == CoveredByFutureSnake);

						if free {
							let seed_coord = coord.go_towards(dir);
							// Check if the node is actually taken
							if !self.is_tree_node_taken(seed_coord) {
								// Seed the tree if all the conditions are met
								self.seed_tree_from(coord, dir);

								if allow_covered {
									seeded_covered_edge = true;
								}

								continue 'iter;
							}
						}
					}
				}
			}

			// We iterate twice, once without allowing covered edges, and once with.
			if !allow_covered {
				allow_covered = true;
				continue;
			} else {
				break;
			}
		}

		if seeded_covered_edge {
			SnakeGrowResult::SuccessWithPathOverride
		} else {
			SnakeGrowResult::Success
		}
	}

	/// Seed the tree from a node and a direction. This begins a depth first search minimum spanning
	/// tree seeding process, and fills all the available space in the region.
	fn seed_tree_from(&mut self, coord: Coord, dir: Direction) {
		self.set_edge(coord, dir, SpanTreeEdgeType::Wall);

		// Leave a small vector for caching directions later
		let mut possible_dirs = Vec::with_capacity(4);

		let mut last_coord = coord.go_towards(dir);

		// We track the depth first search previous locations with a stack
		let mut stack = VecDeque::new();
		stack.push_back(last_coord);

		loop {
			let current_coord = last_coord;

			// Check all directions and add potential valid directions to the possible_dirs vector
			for dir in Direction::each() {
				let next_coord = current_coord.go_towards(dir);
				if !self.is_in_bounds(next_coord) {
					continue;
				}

				if !self.is_tree_node_taken(next_coord) {
					possible_dirs.push(dir);
				}
			}

			// If we reached a dead end, step backwards
			if possible_dirs.len() == 0 {
				last_coord = match stack.pop_back() {
					Some(coord) => coord,
					None => break,
				};
				continue;
			}

			// Pick a random direction from the list
			let dir = possible_dirs[rand::thread_rng().gen_range(0..possible_dirs.len())];
			// let dir = possible_dirs[0];
			possible_dirs.clear();

			// Go towards the chosen direction
			let next_coord = current_coord.go_towards(dir);
			stack.push_back(next_coord);
			last_coord = next_coord;

			self.set_edge(current_coord, dir, SpanTreeEdgeType::Wall);
		}
	}

	/// Check if the snake can pathfind from the current cell outwards. It can can't do it if the next
	/// node is taken and the connecting edge isn't, which implies that the new connection may cause a loop.
	pub fn can_walk_out_from(&self, coord: Coord) -> bool {
		let (coord, dir) = calculate_following_out_edge(coord);

		// Check if the outwards edge is taken
		let connecting_edge = self.graph.get_edge(coord, dir);
		let connecting_edge_taken = match connecting_edge {
			Some(edge) => edge.is_taken(),
			_ => false,
		};

		// Check if the outwards edge's next node is taken by checking all of the edges on the node
		let next_cell = coord.go_towards(dir);
		let next_cell_taken = self.is_tree_node_taken(next_cell);

		// If the next node isn't taken, it's ok
		if !next_cell_taken {
			return true;
		}

		// If the next node is taken but the connecting edge is also taken, it's ok
		if connecting_edge_taken {
			return true;
		}

		return false;
	}

	// Given a coordinate, check if the there's an edge blocking clockwise motion
	pub fn can_walk_clockwise_from(&self, coord: Coord) -> bool {
		let [clockwise, _] = get_valid_dirs_from_coord(coord);
		let (coord, dir) = calculate_inner_tree_coord(coord, clockwise);

		let connecting_edge = self.graph.get_edge(coord, dir);
		let result = match connecting_edge {
			Some(edge) => edge.is_free(),
			_ => true,
		};

		result
	}

	// Given a node's coordinate, check if it has any taken edges (signifying that the node itself is taken)
	pub fn is_tree_node_taken(&self, coord: Coord) -> bool {
		for dir in Direction::each() {
			if let Some(edge) = self.get_edge(coord, dir) {
				if edge.is_taken() {
					return true;
				}
			}
		}

		false
	}

	/// Starting from the snake's head and ending at the snake's food, follow the clockwise
	/// stepping rules until we reach the food. Return the final path.
	pub fn build_snake_path(&self, world: &SnakeWorld) -> Path {
		let mut current = world.snake_head_coord();

		let mut path = Path::new();
		loop {
			if let Some(Cell::Food) = world.get_cell(current) {
				// If we reached the food, we're done
				break;
			}

			let [clockwise, out] = get_valid_dirs_from_coord(current);

			let (coord, dir) = calculate_following_out_edge(current);
			let out_edge = self.get_edge(coord, dir);

			// If the outwards edge is taken then we go out, otherwise we can go clockwise
			let next_dir = if let Some(&SpanTreeEdgeType::Wall) = out_edge {
				out
			} else {
				clockwise
			};

			// Navigate towards the chosen direction
			current = current.go_towards(next_dir);
			path.push(next_dir);
		}

		path
	}

	/// Convert the minimum spanning tree into a collision grid. This was used in the past to
	/// trace the snake path, but now it's just used for debugging and rendering the overlay easier.
	pub fn build_collision_grid_from_walls(&self) -> GridGraph<bool> {
		let mut snake_grid = GridGraph::new(self.size() * 2, false);

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
			let pos = pos.go_towards(dir);
			snake_grid.set_edge(pos, left, true);
		};

		for coord in self.iter_all_coords() {
			for dir in Direction::each() {
				if self.get_edge(coord, dir) == Some(&SpanTreeEdgeType::Wall) {
					set_snake_grid_edge(coord, dir);
				}
			}
		}

		snake_grid
	}

	pub fn clear(&mut self) {
		self.graph.fill(SpanTreeEdgeType::Free);
	}
}

impl std::ops::Deref for SpanningTree {
	type Target = GridGraph<SpanTreeEdgeType>;

	fn deref(&self) -> &Self::Target {
		&self.graph
	}
}

impl std::ops::DerefMut for SpanningTree {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.graph
	}
}
