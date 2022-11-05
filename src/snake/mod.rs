use rand::Rng;

use crate::array2d::Array2D;

use crate::{auto::Path, Coord, Offset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Snake(u32),
	Food,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn each() -> impl Iterator<Item = Self> {
		[Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
	}

	pub fn opposite(&self) -> Self {
		match self {
			Self::Up => Self::Down,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
			Self::Right => Self::Left,
		}
	}

	pub fn rotate_left(&self) -> Self {
		match self {
			Self::Up => Self::Left,
			Self::Down => Self::Right,
			Self::Left => Self::Down,
			Self::Right => Self::Up,
		}
	}

	pub fn rotate_right(&self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
			Self::Right => Self::Down,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnakeResult {
	Stepped,
	Killed,
	Finished,
}

pub struct SnakeWorld {
	snake_length: u32,
	head_coord: Coord,
	food_coord: Coord,
	prev_direction: Option<Direction>,
	cells: Array2D<Cell>,
}

impl SnakeWorld {
	pub fn new(size: usize) -> Self {
		let mut cells = Array2D::new(size, Cell::Empty);

		let head_coord = Coord::new_usize(size / 2, size / 2);
		cells.set(head_coord, Cell::Snake(0));

		let mut world = Self {
			snake_length: 5,
			head_coord,
			food_coord: Coord::new_i32(-1, -1),
			prev_direction: None,
			cells,
		};

		world.spawn_food();

		world
	}

	pub fn step_snake(&mut self, direction: Direction) -> SnakeResult {
		let new_head_coord = match direction {
			Direction::Up => self.head_coord + Offset::up(),
			Direction::Down => self.head_coord + Offset::down(),
			Direction::Left => self.head_coord + Offset::left(),
			Direction::Right => self.head_coord + Offset::right(),
		};

		match self.cells.get(new_head_coord) {
			// Out of bounds
			None => SnakeResult::Killed,

			Some(Cell::Snake(_)) => SnakeResult::Killed,

			Some(Cell::Empty) => {
				self.head_coord = new_head_coord;
				self.cells
					.set(new_head_coord, Cell::Snake(self.snake_length));
				self.cull_tail();

				self.prev_direction = Some(direction);
				SnakeResult::Stepped
			}

			Some(Cell::Food) => {
				self.snake_length += 3;

				self.head_coord = new_head_coord;
				self.cells
					.set(new_head_coord, Cell::Snake(self.snake_length));
				self.cull_tail();

				self.prev_direction = Some(direction);
				if self.spawn_food() {
					SnakeResult::Stepped
				} else {
					SnakeResult::Finished
				}
			}
		}
	}

	fn cull_tail(&mut self) {
		// TODO: Optimize this
		for x in 0..self.cells.size() {
			for y in 0..self.cells.size() {
				let coord = Coord::new_usize(x, y);
				if let Some(Cell::Snake(iteration)) = self.cells.get(coord) {
					if *iteration > 0 {
						self.cells.set(coord, Cell::Snake(iteration - 1));
					} else {
						self.cells.set(coord, Cell::Empty);
					}
				}
			}
		}
	}

	fn find_random_valid_food_coord(&self) -> Option<Coord> {
		if (self.snake_length as usize) < self.cells.count() * 7 / 8 {
			// If more than an eighth of the grid is empty, randomly probe until empty cell found
			let mut rng = rand::thread_rng();
			let mut coord = Coord::new_usize(
				rng.gen_range(0..self.cells.size()),
				rng.gen_range(0..self.cells.size()),
			);

			while self.cells.get(coord) != Some(&Cell::Empty) {
				coord = Coord::new_usize(
					rng.gen_range(0..self.cells.size()),
					rng.gen_range(0..self.cells.size()),
				);
			}

			Some(coord)
		} else {
			// If less than an eighth of the grid is empty, iterate through all cells until empty cell found
			let mut cells = Vec::new();
			for x in 0..self.cells.size() {
				for y in 0..self.cells.size() {
					let coord = Coord::new_usize(x, y);
					if self.cells.get(coord) == Some(&Cell::Empty) {
						cells.push(coord);
					}
				}
			}

			if cells.is_empty() {
				None
			} else {
				let mut rng = rand::thread_rng();
				Some(cells[rng.gen_range(0..cells.len())])
			}
		}
	}

	fn spawn_food(&mut self) -> bool {
		if let Some(coord) = self.find_random_valid_food_coord() {
			self.food_coord = coord;
			self.cells.set(coord, Cell::Food);
			true
		} else {
			false
		}
	}

	pub fn get_cell(&self, coord: Coord) -> Option<&Cell> {
		self.cells.get(coord)
	}

	pub fn snake_head_coord(&self) -> Coord {
		self.head_coord
	}

	pub fn size(&self) -> usize {
		self.cells.size()
	}

	pub fn snake_length(&self) -> u32 {
		self.snake_length
	}

	pub fn food_coord(&self) -> Coord {
		self.food_coord
	}

	pub fn prev_direction(&self) -> Option<Direction> {
		self.prev_direction
	}

	pub fn calculate_snake_path_from_head(&self) -> Path {
		let mut pos = self.head_coord;
		let mut path = Path::new();

		struct NextSnakeCellData {
			coord: Coord,
			direction: Direction,
			value: u32,
		}

		let mut next_snake_cell_data: Option<NextSnakeCellData> = None;
		let Some(Cell::Snake( mut  current_value)) = self.cells.get(pos) else {
			unreachable!("Head coord is not a snake cell")
		};

		loop {
			for direction in Direction::each() {
				let coord = pos + Offset::from_direction(direction);
				if let Some(Cell::Snake(iteration)) = self.cells.get(coord) {
					let smaller = *iteration < current_value;
					if smaller {
						let is_next = if let Some(next_data) = &next_snake_cell_data {
							*iteration > next_data.value
						} else {
							true
						};

						if is_next {
							next_snake_cell_data = Some(NextSnakeCellData {
								coord,
								direction,
								value: *iteration,
							});
						}
					}
				}
			}

			if let Some(next_data) = next_snake_cell_data {
				path.push(next_data.direction);
				pos = next_data.coord;
				current_value = next_data.value;
				next_snake_cell_data = None;
			} else {
				break;
			}
		}

		path
	}
}
