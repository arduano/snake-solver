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
	const UNINITIALIZED: Self;

	fn has_been_visited(&self) -> bool {
		*self != Self::UNINITIALIZED
	}
}

impl PathfindValue for u32 {
	const MARKED: Self = u32::MAX;
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
		let grid = Array2D::new(world_size, u32::UNINITIALIZED);
		Self { grid }
	}

	pub fn fill_pathfinding_grid(&mut self, world: &SnakeWorld, graph: &SpanningTree) {
		let starting_coord = world.food_coord();

		let mut queue = VecDeque::new();
		queue.push_back((starting_coord, 1));

		while let Some((coord, dist)) = queue.pop_front() {
			self.set(coord, dist);

			let [clockwise, out] = get_valid_dirs_from_coord(coord);
			let items = [
				(clockwise.opposite(), GridStepKind::Out),
				(out.opposite(), GridStepKind::Clockwise),
			];

			for (dir, kind) in items.into_iter() {
				let next = coord.go_towards(dir);

				let Some(cell) = world.get_cell(next) else {
						continue;
					};

				if let Cell::Snake(_) = cell {
					continue;
				}

				let Some(&current_val) = self.get(next) else {
						continue;
					};

				if current_val.has_been_visited() {
					continue;
				}

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

				self.set(next, u32::MARKED);
				queue.push_back((next, dist + 1));
			}
		}
	}

	pub fn clear(&mut self) {
		self.grid.fill(u32::UNINITIALIZED);
	}
}

pub fn pathfind_on_spanning_tree(
	from: Coord,
	grid: &PathfindingGrid,
	tree: &mut SpanningTree,
) -> SnakePathfindResult {
	let mut current = from;

	loop {
		let current_value = *grid.get(current).unwrap();
		if current_value == 1 {
			break;
		}

		let [clockwise, out] = get_valid_dirs_from_coord(current);

		let get_value_at = |dir: Direction| {
			let coord = current.go_towards(dir);
			grid.get(coord)
				.and_then(|&v| if v == 0 { None } else { Some(v) })
		};

		let clockwise_value = get_value_at(clockwise);
		let out_value = get_value_at(out);

		if clockwise_value == None && out_value == None {
			return SnakePathfindResult::ReachedDeadEnd;
		}

		let next_dir = if clockwise_value == None {
			GridStepKind::Out
		} else if out_value == None {
			GridStepKind::Clockwise
		} else if clockwise_value < out_value {
			GridStepKind::Clockwise
		} else if tree.can_walk_out_from(current) {
			GridStepKind::Out
		} else {
			GridStepKind::Clockwise
		};

		if next_dir == GridStepKind::Out {
			let (coord, dir) = calculate_following_out_edge(current);
			tree.try_set_edge(coord, dir, SpanTreeEdgeType::Wall);
		}

		if next_dir == GridStepKind::Clockwise {
			let (coord, dir) = calculate_following_out_edge(current);
			tree.try_set_edge(coord, dir, SpanTreeEdgeType::CoveredByFutureSnake);
		}

		let next_dir = match next_dir {
			GridStepKind::Clockwise => clockwise,
			GridStepKind::Out => out,
		};

		current = current.go_towards(next_dir);
	}

	SnakePathfindResult::Success
}
