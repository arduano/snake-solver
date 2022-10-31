mod path;
use std::marker::PhantomData;

pub use path::*;

use crate::{
    snake::{SnakeResult, SnakeWorld},
    solvers::SnakeSolver,
};

pub struct AutoSnakePlayer<S: SnakeSolver> {
    world: SnakeWorld,
    current_path: Path,
    ended: bool,
    _s: PhantomData<S>,
}

impl<S: SnakeSolver> AutoSnakePlayer<S> {
    pub fn new(size: usize) -> Self {
        let world = SnakeWorld::new(size);
        let initial_path = S::get_next_path(&world);
        Self {
            world,
            current_path: initial_path,
            ended: false,
            _s: PhantomData,
        }
    }

    pub fn step(&mut self) -> SnakeResult {
        if self.ended {
            return SnakeResult::Finished;
        }

        let next_step = loop {
            if let Some(next) = self.current_path.pop() {
                break next;
            } else {
                self.current_path = S::get_next_path(&self.world);
                if self.current_path.is_empty() {
                    panic!("Solver returned empty path");
                }
            }
        };

        let result = self.world.step_snake(next_step);

        if result == SnakeResult::Finished {
            self.ended = true;
        }

        result
    }

    pub fn world(&self) -> &SnakeWorld {
        &self.world
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }
}
