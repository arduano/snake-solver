use crate::{
	grid_graph::GridGraph,
	path::Path,
	snake::{Cell, Direction, SnakeWorld},
	Coord, Offset,
};

pub fn get_valid_dirs_from_coord(coord: Coord) -> [Direction; 2] {
	let twos_coords = [coord.x % 2, coord.y % 2];
	let [clockwise, out] = match twos_coords {
		[0, 0] => [Direction::Right, Direction::Up],
		[1, 0] => [Direction::Down, Direction::Right],
		[1, 1] => [Direction::Left, Direction::Down],
		[0, 1] => [Direction::Up, Direction::Left],
		_ => unreachable!(),
	};

	[clockwise, out]
}

pub fn build_path_from_collision_grid(grid: &GridGraph<bool>, world: &SnakeWorld) -> Path {
	let mut current = world.snake_head_coord();

	let mut path = Path::new();
	loop {
		if let Some(Cell::Food) = world.get_cell(current) {
			break;
		}

		let [clockwise, out] = get_valid_dirs_from_coord(current);

		let next_dir = if grid.get_edge(current, clockwise) == Some(&false) {
			clockwise
		} else {
			out
		};

		current = current + Offset::from_direction(next_dir);
		path.push(next_dir);
	}

	path
}
