#![allow(dead_code)]

use crate::snake::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord {
	pub x: i32,
	pub y: i32,
}

impl Coord {
	pub fn new_i32(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	pub fn new_usize(x: usize, y: usize) -> Self {
		Self {
			x: x as i32,
			y: y as i32,
		}
	}

	pub fn get_offset(&self, other: Coord) -> Offset {
		Offset::new(self.x - other.x, self.y - other.y)
	}

	fn add_offset(&self, offset: Offset) -> Self {
		Self {
			x: self.x + offset.0.x,
			y: self.y + offset.0.y,
		}
	}

	pub fn map_values(&self, f: impl Fn(i32) -> i32) -> Self {
		Self {
			x: f(self.x),
			y: f(self.y),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Offset(Coord);

impl Offset {
	pub fn new(x: i32, y: i32) -> Self {
		Self(Coord::new_i32(x, y))
	}

	pub fn zero() -> Self {
		Self::new(0, 0)
	}

	pub fn up() -> Self {
		Self::new(0, -1)
	}

	pub fn down() -> Self {
		Self::new(0, 1)
	}

	pub fn left() -> Self {
		Self::new(-1, 0)
	}

	pub fn right() -> Self {
		Self::new(1, 0)
	}

	pub fn each() -> impl Iterator<Item = Self> {
		[Self::up(), Self::down(), Self::left(), Self::right()].into_iter()
	}

	pub fn from_direction(dir: Direction) -> Self {
		match dir {
			Direction::Up => Self::up(),
			Direction::Down => Self::down(),
			Direction::Left => Self::left(),
			Direction::Right => Self::right(),
		}
	}

	fn add_offset(&self, offset: Offset) -> Self {
		Self(self.0.add_offset(offset))
	}

	fn length(&self) -> i32 {
		f32::sqrt((self.x * self.x + self.y * self.y) as f32) as i32
	}
}

impl std::ops::Deref for Offset {
	type Target = Coord;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for Offset {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl std::ops::Add<Coord> for Offset {
	type Output = Coord;

	fn add(self, rhs: Coord) -> Self::Output {
		rhs.add_offset(self)
	}
}

impl std::ops::Add<Offset> for Coord {
	type Output = Coord;

	fn add(self, rhs: Offset) -> Self::Output {
		self.add_offset(rhs)
	}
}

impl std::ops::Add<Offset> for Offset {
	type Output = Offset;

	fn add(self, rhs: Offset) -> Self::Output {
		self.add_offset(rhs)
	}
}

impl std::ops::AddAssign<Offset> for Coord {
	fn add_assign(&mut self, rhs: Offset) {
		self.x += rhs.0.x;
		self.y += rhs.0.y;
	}
}

impl std::ops::AddAssign<Offset> for Offset {
	fn add_assign(&mut self, rhs: Offset) {
		self.0.x += rhs.0.x;
		self.0.y += rhs.0.y;
	}
}
