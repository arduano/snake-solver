use eframe::egui::{self, Response, Sense, Widget};

use crate::{
	auto::Path,
	snake::{self, SnakeWorld},
	Coord,
};

pub struct SnakeWorldViewer<'a> {
	snake_world: &'a SnakeWorld,
	overlay_path: Option<&'a Path>,
}

impl<'a> SnakeWorldViewer<'a> {
	pub fn new(snake_world: &'a SnakeWorld) -> Self {
		Self {
			snake_world,
			overlay_path: None,
		}
	}

	pub fn with_path_overlay(mut self, path: &'a Path) -> Self {
		self.overlay_path = Some(path);
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

		// Add background
		mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 0, 0));

		for x in 0..self.snake_world.size() {
			for y in 0..self.snake_world.size() {
				let coord = Coord::new_usize(x, y);
				let cell = self.snake_world.get_cell(coord).copied();
				let rect = egui::Rect::from_min_size(
					rect.min + egui::vec2(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE),
					egui::vec2(CELL_SIZE, CELL_SIZE),
				);
				match cell {
					Some(snake::Cell::Empty) => {}
					Some(snake::Cell::Snake(_)) => {}
					Some(snake::Cell::Food) => {
						// We only add food here
						mesh.add_colored_rect(rect, egui::Color32::from_rgb(255, 0, 0));
					}

					None => unreachable!(),
				}
			}
		}

		let mut iter = snake_path.iter_offsets();
		let head = self.snake_world.snake_head_coord();
		let mut prev: Option<Coord> = None;

		let get_rect_for_coord = |coord: Coord| {
			let padding = 1.0;
			egui::Rect::from_min_size(
				rect.min
					+ egui::vec2(
						coord.x as f32 * CELL_SIZE + padding,
						coord.y as f32 * CELL_SIZE + padding,
					),
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

				let half_cell = egui::vec2(CELL_SIZE / 2.0, CELL_SIZE / 2.0);

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

		response
	}
}
