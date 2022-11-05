use crate::{auto::Path, snake::SnakeWorld};

pub mod basic;
pub mod random_spanning_tree;
mod utils;

pub trait SnakeSolver {
	fn get_next_path(world: &SnakeWorld) -> Path;
}
