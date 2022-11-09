use crate::{direction::Direction, solvers::utils::get_valid_dirs_from_coord, Coord, Offset};

/// Assuming coord is a coordinate in the snake world and dir is a direction, return the spanning
/// tree edge location that the coordinate and direction are pointing at.
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

/// Assuming coord is a coordinate in the snake world, return the spanning
/// tree edge location that we'll be moving along if we moved outwards. It is also the
/// edge we colide with if we move clockwise.
pub fn calculate_following_out_edge(coord: Coord) -> (Coord, Direction) {
	let meta_coord = coord.map_values(|v| v / 2);
	let [_, out] = get_valid_dirs_from_coord(coord);
	(meta_coord, out)
}
