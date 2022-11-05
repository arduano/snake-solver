use crate::{array2d::Array2D, snake::Direction, Coord, Offset};

pub struct GridGraph<T> {
	size: usize,
	cells: Array2D<T>,
}

impl<T> GridGraph<T> {
	pub fn new(size: usize, default: T) -> Self
	where
		T: Clone,
	{
		let true_size = size * 2 - 1;
		Self {
			size,
			cells: Array2D::new(true_size, default),
		}
	}

	fn get_cell_coord(&self, coord: Coord) -> Coord {
		coord.map_values(|v| v * 2)
	}

	fn get_edge_coord(&self, coord: Coord, dir: Direction) -> Coord {
		self.get_cell_coord(coord) + Offset::from_direction(dir)
	}

	pub fn get_cell(&self, coord: Coord) -> Option<&T> {
		self.cells.get(self.get_cell_coord(coord))
	}

	pub fn get_edge(&self, coord: Coord, dir: Direction) -> Option<&T> {
		self.cells.get(self.get_edge_coord(coord, dir))
	}

	pub fn set_cell(&mut self, coord: Coord, value: T) {
		self.cells.set(self.get_cell_coord(coord), value);
	}

	pub fn set_edge(&mut self, coord: Coord, dir: Direction, value: T) {
		self.cells.set(self.get_edge_coord(coord, dir), value);
	}

	pub fn is_in_bounds(&self, coord: Coord) -> bool {
		coord.x >= 0 && coord.y >= 0 && coord.x < self.size as i32 && coord.y < self.size as i32
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn count(&self) -> usize {
		self.size * self.size
	}
}
