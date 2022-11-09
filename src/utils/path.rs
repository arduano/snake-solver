use std::collections::VecDeque;

use crate::{direction::Direction, Offset};

#[derive(Debug, Clone)]
pub struct Path {
	directions: VecDeque<Direction>,
}

impl Path {
	pub fn new() -> Self {
		Self {
			directions: VecDeque::new(),
		}
	}

	pub fn push(&mut self, direction: Direction) {
		self.directions.push_back(direction);
	}

	pub fn pop(&mut self) -> Option<Direction> {
		self.directions.pop_front()
	}

	pub fn is_empty(&self) -> bool {
		self.directions.is_empty()
	}

	pub fn iter_offsets(&self) -> impl '_ + Iterator<Item = Offset> {
		let first = std::iter::once(Offset::zero());

		let rest = self
			.directions
			.iter()
			.scan(Offset::zero(), |current, direction| {
				*current += Offset::from_direction(*direction);
				Some(*current)
			});

		first.chain(rest)
	}

	pub fn iter_directions(&self) -> impl '_ + Iterator<Item = Direction> {
		self.directions.iter().copied()
	}
}
