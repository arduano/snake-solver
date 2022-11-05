use eframe::egui::{self, Response, Sense, Widget};

use crate::{
	auto::Path,
	grid_graph::GridGraph,
	snake::{Direction, SnakeWorld},
	Coord, Offset,
};

pub struct SnakeWorldViewer<'a> {
	snake_world: &'a SnakeWorld,
	overlay_path: Option<&'a Path>,
	bools_edges_grid: Option<&'a GridGraph<bool>>,
}

impl<'a> SnakeWorldViewer<'a> {
	pub fn new(snake_world: &'a SnakeWorld) -> Self {
		Self {
			snake_world,
			overlay_path: None,
			bools_edges_grid: None,
		}
	}

	pub fn with_path_overlay(mut self, path: &'a Path) -> Self {
		self.overlay_path = Some(path);
		self
	}

	pub fn with_bools_edges_grid_overlay(mut self, bools_edges_grid: &'a GridGraph<bool>) -> Self {
		self.bools_edges_grid = Some(bools_edges_grid);
		self
	}
}

const CELL_SIZE: f32 = 10.0;

impl Widget for SnakeWorldViewer<'_> {
	fn ui(self, ui: &mut eframe::egui::Ui) -> Response {
		let size = self.snake_world.size() as f32 * CELL_SIZE;

		let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), Sense::click());

		let painter = ui.painter();

		let snake_path = self.snake_world.calculate_snake_path_from_head();

		let mut mesh = egui::Mesh::default();

		let get_coord_vec2 = |coord: Coord| {
			rect.min + egui::vec2(coord.x as f32 * CELL_SIZE, coord.y as f32 * CELL_SIZE)
		};
		let half_cell = egui::vec2(CELL_SIZE / 2.0, CELL_SIZE / 2.0);

		// Add background
		mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 0, 0));

		// Add food
		mesh.add_colored_rect(
			egui::Rect::from_min_size(
				get_coord_vec2(self.snake_world.food_coord()),
				egui::vec2(CELL_SIZE, CELL_SIZE),
			),
			egui::Color32::from_rgb(255, 0, 0),
		);

		let mut iter = snake_path.iter_offsets();
		let head = self.snake_world.snake_head_coord();
		let mut prev: Option<Coord> = None;

		let get_rect_for_coord = |coord: Coord| {
			let padding = 1.0;
			egui::Rect::from_min_size(
				get_coord_vec2(coord) + egui::vec2(padding, padding),
				egui::vec2(CELL_SIZE - padding * 2.0, CELL_SIZE - padding * 2.0),
			)
		};

		while let Some(offset) = iter.next() {
			let coord = head + offset;

			let rect = if let Some(prev) = prev {
				get_rect_for_coord(coord).union(get_rect_for_coord(prev))
			} else {
				get_rect_for_coord(coord)
			};

			mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 255, 0));

			prev = Some(coord);
		}

		painter.add(egui::Shape::Mesh(mesh));

		let render_path = |path: &Path, head: Coord, color: egui::Color32| {
			let mut iter = path.iter_offsets();

			// Unwrap is safe here because the offsets iterator always starts with zero
			let mut prev = iter.next().unwrap();

			while let Some(offset) = iter.next() {
				let start = head + prev;
				let end = head + offset;

				let start = rect.min
					+ egui::vec2(start.x as f32 * CELL_SIZE, start.y as f32 * CELL_SIZE)
					+ half_cell;
				let end = rect.min
					+ egui::vec2(end.x as f32 * CELL_SIZE, end.y as f32 * CELL_SIZE)
					+ half_cell;

				painter.add(egui::Shape::line_segment(
					[start, end],
					egui::Stroke::new(1.0, color),
				));

				prev = offset;
			}
		};

		if let Some(path) = &self.overlay_path {
			render_path(
				path,
				self.snake_world.snake_head_coord(),
				egui::Color32::from_rgb(255, 255, 0),
			);
		}

		if let Some(bools_edges_grid) = &self.bools_edges_grid {
			for x in 0..bools_edges_grid.size() {
				for y in 0..bools_edges_grid.size() {
					let coord = Coord::new_usize(x, y);
					for dir in [Direction::Right, Direction::Down].into_iter() {
						if bools_edges_grid.get_edge(coord, dir) == Some(&true) {
							let start = coord + Offset::from_direction(dir);

							let next_dir = match dir {
								Direction::Right => dir.rotate_right(),
								Direction::Down => dir.rotate_left(),
								_ => unreachable!(),
							};

							let end = start + Offset::from_direction(next_dir);

							let start = get_coord_vec2(start);
							let end = get_coord_vec2(end);
							painter.add(egui::Shape::line_segment(
								[start, end],
								egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 0, 255)),
							));
						}
					}
				}
			}
		}

		response
	}
}
