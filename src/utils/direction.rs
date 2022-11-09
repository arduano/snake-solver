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
