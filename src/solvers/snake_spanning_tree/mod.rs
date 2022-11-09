use std::ops::Deref;

use crate::{auto::Path, snake::SnakeWorld, ui::SnakeWorldViewer};

use self::{
	pathfinding::{PathfindingGrid, SnakePathfindResult},
	spanning_tree::{SnakeGrowResult, SpanningTree},
};

use super::SnakeSolver;

mod coordinates;
mod pathfinding;
mod spanning_tree;

pub struct SnakeSpanningTreeSolver {
	spanning_tree: Option<SpanningTree>,
	pathfinding_grid: Option<PathfindingGrid>,
	jitter_setting: JitterKind,
}

pub enum JitterKind {
	NoJitter,
	JitterWhenIndirect(usize),
	JitterAlways(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridStepKind {
	Clockwise,
	Out,
}

impl SnakeSpanningTreeSolver {
	pub fn new(jitter_setting: JitterKind) -> Self {
		Self {
			spanning_tree: None,
			pathfinding_grid: None,
			jitter_setting,
		}
	}
}

impl SnakeSolver for SnakeSpanningTreeSolver {
	fn get_next_path(&mut self, world: &SnakeWorld) -> Path {
		// Fetch the cached data structures to avoid re-allocations
		let spanning_tree = self
			.spanning_tree
			.get_or_insert_with(|| SpanningTree::new(world.size()));
		let pathfinding_grid = self
			.pathfinding_grid
			.get_or_insert_with(|| PathfindingGrid::new(world.size()));

		// Clear them just in case after fetching
		spanning_tree.clear();
		pathfinding_grid.clear();

		// Step 1: Trace the snake into the spanning tree
		spanning_tree.trace_current_snake_and_mark_edges(&world);

		// Step 2: Fill the pathfinding grid from the spanning tree
		pathfinding_grid.fill_pathfinding_grid(world, spanning_tree);

		// Step 3: Pathfind through the grid, extending the tree
		let pathfind_result = pathfinding::pathfind_on_spanning_tree(
			world.snake_head_coord(),
			pathfinding_grid,
			spanning_tree,
		);

		// Process the pathfind result. Theoretically a dead end can never be reached,
		// but we handle it just in case.
		match pathfind_result {
			SnakePathfindResult::Success => {}
			SnakePathfindResult::ReachedDeadEnd => {
				// Theoretically this can't be reached, but we handle it just in case.

				println!("Reached dead end while pathfinding");
				println!("Pathfinding failed, returning a killing path");

				// Return a path that goes backwards into the snake to kill it
				let mut path = Path::new();
				path.push(
					world
						.calculate_snake_path_from_head()
						.iter_directions()
						.nth(0)
						.unwrap(),
				);
				return path;
			}
		}

		// Step 4: Grow the spanning tree to fill the remaining space
		let grow_result = spanning_tree.grow_spanning_tree();

		// Step 5: Trace the spanning tree to create the snake path
		let path = spanning_tree.build_snake_path(&world);

		// Handle the growth result. We choose different step counts depending on the result and the jitter setting.
		let take = match grow_result {
			SnakeGrowResult::Success => {
				if let JitterKind::JitterWhenIndirect(num) = self.jitter_setting {
					Some(num)
				} else {
					None
				}
			}
			SnakeGrowResult::SuccessWithPathOverride => match self.jitter_setting {
				JitterKind::JitterWhenIndirect(num) => Some(num),
				JitterKind::JitterAlways(num) => Some(num),
				JitterKind::NoJitter => None,
			},
		};

		// Shorten the path if necessary
		let path = if let Some(take) = take {
			let mut clipped_path = Path::new();

			for dir in path.iter_directions().take(take) {
				clipped_path.push(dir);
			}

			clipped_path
		} else {
			path
		};

		path
	}

	// UI function to decorate the widget with pathfinding metadata
	fn decorate_widget<'a>(&'a self, mut widget: SnakeWorldViewer<'a>) -> SnakeWorldViewer<'a> {
		if let Some(tree) = &self.spanning_tree {
			widget = widget.with_bools_edges_grid_overlay(
				tree.build_collision_grid_from_walls(),
				eframe::egui::Color32::from_rgb(0, 255, 255),
			);
		}

		if let Some(prev_pathfinding_grid) = &self.pathfinding_grid {
			widget = widget.with_pathfinding_grid_overlay(prev_pathfinding_grid.deref());
		}

		widget
	}
}
