use crate::{auto::Path, snake::SnakeWorld, ui::SnakeWorldViewer};

pub mod basic;
pub mod random_spanning_tree;
pub mod snake_spanning_tree;
mod utils;

pub trait SnakeSolver {
	fn get_next_path(&mut self, world: &SnakeWorld) -> Path;
	fn decorate_widget<'a>(&'a self, widget: SnakeWorldViewer<'a>) -> SnakeWorldViewer<'a>;
}
