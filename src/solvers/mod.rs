use crate::{auto::Path, snake::SnakeWorld};

pub mod basic;

pub trait SnakeSolver {
    fn get_next_path(world: &SnakeWorld) -> Path;
}
