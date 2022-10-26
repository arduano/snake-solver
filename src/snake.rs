use rand::Rng;

use crate::array2d::{Array2D, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Snake(usize),
	Food,
}

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub enum SnakeResult {
	Stepped,
	Killed,
	Finished,
}

pub struct SnakeWorld {
	iteration: usize,
	snake_length: usize,
	head_coord: Coord,
	cells: Array2D<Cell>,
}

impl SnakeWorld {
	pub fn new(size: usize) -> Self {
		let snake_length = 1;
		let starting_iteration = 0;
		let mut cells = Array2D::new(size, Cell::Empty);

		let head_coord = Coord::new(size / 2, size / 2);
		cells.set(head_coord, Cell::Snake(starting_iteration));

		let mut world = Self {
			iteration: starting_iteration,
			snake_length,
			head_coord,
			cells,
		};

		world.spawn_food();

		world
	}

	pub fn step_snake(&mut self, direction: Direction) -> SnakeResult {
		let new_head_coord = match direction {
			Direction::Up => match self.head_coord.up() {
				Some(coord) => coord,
				None => return SnakeResult::Killed,
			},
			Direction::Down => self.head_coord.down(),
			Direction::Left => match self.head_coord.left() {
				Some(coord) => coord,
				None => return SnakeResult::Killed,
			},
			Direction::Right => self.head_coord.right(),
		};

		match self.cells.get(new_head_coord) {
			// Out of bounds
			None => SnakeResult::Killed,

			Some(Cell::Snake(_)) => SnakeResult::Killed,

			Some(Cell::Empty) => {
				self.iteration += 1;
				self.head_coord = new_head_coord;
				self.cells.set(new_head_coord, Cell::Snake(self.snake_length));
				self.cull_tail();
				SnakeResult::Stepped
			}

			Some(Cell::Food) => {
				self.iteration += 1;
				self.snake_length += 1;

				if self.snake_length == self.cells.count() {
					SnakeResult::Finished
				} else {
					self.head_coord = new_head_coord;
					self.cells.set(new_head_coord, Cell::Snake(self.snake_length));
					self.cull_tail();
					self.spawn_food();
					SnakeResult::Stepped
				}
			}
		}
	}

	fn cull_tail(&mut self) {
		// TODO: Optimize this
		for x in 0..self.cells.size() {
			for y in 0..self.cells.size() {
				let coord = Coord::new(x, y);
				if let Some(Cell::Snake(iteration)) = self.cells.get(coord) {
					if *iteration > 0 as usize {
						self.cells.set(coord, Cell::Snake(iteration-1));
					} else {
						self.cells.set(coord, Cell::Empty);
					}
				}
			}
		}
	}

	fn find_random_valid_food_coord(&self) -> Option<Coord> {
		//TODO: If there are only a few empty cells, form an array of them instead and pick from that
		if self.snake_length == self.cells.count() {
			return None;
		}

		let mut rng = rand::thread_rng();
		let mut coord = Coord::new(
			rng.gen_range(0..self.cells.size()),
			rng.gen_range(0..self.cells.size()),
		);

		while self.cells.get(coord) != Some(&Cell::Empty) {
			coord = Coord::new(
				rng.gen_range(0..self.cells.size()),
				rng.gen_range(0..self.cells.size()),
			);
		}

		Some(coord)
	}

	fn spawn_food(&mut self) {
		if let Some(coord) = self.find_random_valid_food_coord() {
			self.cells.set(coord, Cell::Food);
		}
	}

	pub fn get_cell(&self, coord: Coord) -> Option<&Cell> {
		self.cells.get(coord)
	}

	pub fn size(&self) -> usize {
		self.cells.size()
	}
}
