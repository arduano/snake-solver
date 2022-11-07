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

	pub fn try_set_edge(&mut self, coord: Coord, dir: Direction, value: T) -> bool {
		let edge_coord = self.get_edge_coord(coord, dir);
		if self.cells.is_in_bounds(edge_coord) {
			self.cells.set(edge_coord, value);
			true
		} else {
			false
		}
	}

	pub fn try_set_cell(&mut self, coord: Coord, value: T) -> bool {
		let cell_coord = self.get_cell_coord(coord);
		if self.cells.is_in_bounds(cell_coord) {
			self.cells.set(cell_coord, value);
			true
		} else {
			false
		}
	}

	pub fn is_in_bounds(&self, coord: Coord) -> bool {
		let edge_coord = self.get_cell_coord(coord);
		self.cells.is_in_bounds(edge_coord)
	}

	pub fn is_in_bounds_with_direction(&self, coord: Coord, direction: Direction) -> bool {
		let edge_coord = self.get_edge_coord(coord, direction);
		self.cells.is_in_bounds(edge_coord)
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn count(&self) -> usize {
		self.size * self.size
	}
}
