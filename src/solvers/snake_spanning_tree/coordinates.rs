use crate::{direction::Direction, solvers::utils::get_valid_dirs_from_coord, Coord, Offset};

pub fn calculate_inner_tree_coord(coord: Coord, dir: Direction) -> (Coord, Direction) {
	let meta_coord = coord.map_values(|v| (v + 1) / 2 - 1);

	let meta_coord = match dir {
		Direction::Left | Direction::Up => meta_coord,
		Direction::Right | Direction::Down => meta_coord + Offset::new(1, 1),
	};

	let new_dir = match dir {
		Direction::Left | Direction::Right => dir.rotate_left(),
		Direction::Up | Direction::Down => dir.rotate_right(),
	};

	(meta_coord, new_dir)
}

pub fn calculate_following_out_edge(coord: Coord) -> (Coord, Direction) {
	let meta_coord = coord.map_values(|v| v / 2);
	let [_, out] = get_valid_dirs_from_coord(coord);
	(meta_coord, out)
}
