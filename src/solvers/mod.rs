use crate::{auto::Path, snake::SnakeWorld};

pub mod basic;
pub mod random_spanning_tree;

pub trait SnakeSolver {
	fn get_next_path(&mut self, world: &SnakeWorld) -> Path;
}
