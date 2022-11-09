use std::collections::VecDeque;

use crate::{
	array2d::Array2D,
	direction::Direction,
	snake::{Cell, SnakeWorld},
	solvers::utils::get_valid_dirs_from_coord,
	Coord,
};

use super::{
	coordinates::calculate_following_out_edge,
	spanning_tree::{SpanTreeEdgeType, SpanningTree},
	GridStepKind,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SnakePathfindResult {
	Success,
	ReachedDeadEnd,
}

trait PathfindValue: Eq + Sized {
	const MARKED: Self;
	const START: Self;
	const UNINITIALIZED: Self;

	fn has_been_visited(&self) -> bool {
		*self != Self::UNINITIALIZED
	}
}

impl PathfindValue for u32 {
	const MARKED: Self = u32::MAX;
	const START: Self = 1;
	const UNINITIALIZED: Self = 0;
}

pub struct PathfindingGrid {
	grid: Array2D<u32>,
}

impl std::ops::Deref for PathfindingGrid {
	type Target = Array2D<u32>;

	fn deref(&self) -> &Self::Target {
		&self.grid
	}
}

impl std::ops::DerefMut for PathfindingGrid {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.grid
	}
}

impl PathfindingGrid {
	pub fn new(world_size: usize) -> Self {
		let grid = Array2D::new(world_size, PathfindValue::UNINITIALIZED);
		Self { grid }
	}

	/// Given a snake world and a spanning tree, fill the grid starting from the food coordinate outwards.
	pub fn fill_pathfinding_grid(&mut self, world: &SnakeWorld, graph: &SpanningTree) {
		let starting_coord = world.food_coord();

		// We use a basic queue system for A*
		let mut queue = VecDeque::new();
		queue.push_back((starting_coord, PathfindValue::START));

		while let Some((coord, dist)) = queue.pop_front() {
			// Visit the cell and set the value
			self.set(coord, dist);

			// Get the valid locations that a snake could come from into this cell.
			// We use .opposite and reverse the out/clockwise because we're checking where
			// the snake can come from, not go towards.
			let [clockwise, out] = get_valid_dirs_from_coord(coord);
			let items = [
				(clockwise.opposite(), GridStepKind::Out),
				(out.opposite(), GridStepKind::Clockwise),
			];

			for (dir, kind) in items.into_iter() {
				// Verify the cell we're checking
				let next = coord.go_towards(dir);

				let Some(cell) = world.get_cell(next) else {
					// If out of bounds, continue
					continue;
				};

				if let Cell::Snake(_) = cell {
					// If occupied by the current snake, continue
					continue;
				}

				let Some(&current_val) = self.get(next) else {
					// If out of bounds, continue
					continue;
				};

				if current_val.has_been_visited() {
					// If has already been visited, continue
					continue;
				}

				// Check if the move is valid
				match kind {
					GridStepKind::Clockwise => {
						if !graph.can_walk_clockwise_from(next) {
							continue;
						}
					}
					GridStepKind::Out => {
						if !graph.can_walk_out_from(next) {
							continue;
						}
					}
				}

				// If everything is valid, we mark the cell and add it to the queue.
				self.set(next, PathfindValue::MARKED);
				queue.push_back((next, dist + 1));
			}
		}
	}

	pub fn clear(&mut self) {
		self.grid.fill(PathfindValue::UNINITIALIZED);
	}
}

/// Pathfind along the grid along valid directions, placing new spanning tree edges until the food is reached.
pub fn pathfind_on_spanning_tree(
	from: Coord,
	grid: &PathfindingGrid,
	tree: &mut SpanningTree,
) -> SnakePathfindResult {
	let mut current = from;

	loop {
		let current_value = *grid.get(current).unwrap();
		if current_value == PathfindValue::START {
			// If we reached the minimum value, we're done.
			break;
		}

		// Get the valid directions from the coord
		let [clockwise, out] = get_valid_dirs_from_coord(current);

		let get_value_at = |dir: Direction| {
			let coord = current.go_towards(dir);
			grid.get(coord).and_then(|&v| {
				if v == PathfindValue::UNINITIALIZED {
					None
				} else {
					Some(v)
				}
			})
		};

		// Get the vales in the valid directions. If a cell is out of bounds or uninitialized, then it's value is None.
		let clockwise_value = get_value_at(clockwise);
		let out_value = get_value_at(out);

		// If both are None, then we've reached a dead end. This shouldn't be possible, but we handle it anyway.
		if clockwise_value == None && out_value == None {
			return SnakePathfindResult::ReachedDeadEnd;
		}

		let next_dir = if clockwise_value == None {
			// If we can't go clockwise, then go out
			GridStepKind::Out
		} else if out_value == None {
			// If we can't go out, then go clockwise
			GridStepKind::Clockwise
		} else if clockwise_value < out_value {
			// If clockwise is smaller then out, go clockwise
			GridStepKind::Clockwise
		} else if tree.can_walk_out_from(current) {
			// Otherwise go out, but only if going out is valid
			GridStepKind::Out
		} else {
			GridStepKind::Clockwise
		};

		// Set the edge accordingly
		let (coord, dir) = calculate_following_out_edge(current);
		match next_dir {
			GridStepKind::Out => {
				tree.try_set_edge(coord, dir, SpanTreeEdgeType::Wall);
			}
			GridStepKind::Clockwise => {
				tree.try_set_edge(coord, dir, SpanTreeEdgeType::CoveredByFutureSnake);
			}
		}

		let next_dir = match next_dir {
			GridStepKind::Clockwise => clockwise,
			GridStepKind::Out => out,
		};

		current = current.go_towards(next_dir);
	}

	SnakePathfindResult::Success
}
