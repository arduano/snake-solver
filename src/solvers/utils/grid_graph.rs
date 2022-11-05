pub struct GridGraph<T> {
	size: usize,
	cells: Array2d<T>,
}

impl<T> GridGraph<T> {
	pub fn new(size: usize, default: T) -> Self
	where
		T: Clone,
	{
		let cells = vec![default; size * size];
		Self { size, cells }
	}

	pub fn get(&self, coord: Coord) -> Option<&T> {
		if !self.is_in_bounds(coord) {
			return None;
		} else {
			Some(&self.cells[coord.y as usize * self.size + coord.x as usize])
		}
	}

	pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
		if !self.is_in_bounds(coord) {
			return None;
		} else {
			Some(&mut self.cells[coord.y as usize * self.size + coord.x as usize])
		}
	}

	pub fn set(&mut self, coord: Coord, value: T) {
		self.cells[coord.y as usize * self.size + coord.x as usize] = value;
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
