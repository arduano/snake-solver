use crate::{
	auto::Path,
	snake::{Cell, Direction},
	ui::SnakeWorldViewer,
	Offset,
};

use super::SnakeSolver;

/// Generates a path that zigzags until the food, then when it reaches the bottom it goes up along the left.
/// This only works in evenly-sized worlds.
pub struct BasicSnakeSolver;

impl SnakeSolver for BasicSnakeSolver {
	fn get_next_path(&mut self, world: &crate::snake::SnakeWorld) -> Path {
		let mut path = Path::new();

		let mut current_coord = world.snake_head_coord();

		loop {
			let current_cell = world.get_cell(current_coord);

			if current_cell == Some(&Cell::Food) {
				break;
			}

			let next_dir = if current_coord.x == 0 {
				if current_coord.y == 0 {
					Direction::Right
				} else {
					Direction::Up
				}
			} else {
				let going_right = current_coord.y % 2 == 0;

				let world_max = (world.size() - 1) as i32;

				if going_right && current_coord.x == world_max {
					// Reached the end of right
					Direction::Down
				} else if !going_right && current_coord.x == 1 && current_coord.y != world_max {
					// Reached the end of left, only if not in the bottom row
					Direction::Down
				} else if going_right {
					Direction::Right
				} else {
					Direction::Left
				}
			};

			path.push(next_dir);
			current_coord += Offset::from_direction(next_dir);
		}

		path
	}

	fn decorate_widget<'a>(&'a self, widget: SnakeWorldViewer<'a>) -> SnakeWorldViewer<'a> {
		widget
	}
}
