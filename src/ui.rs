use eframe::egui::{self, Response, Sense, Widget};
use palette::convert::FromColorUnclamped;

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
		for x in 0..self.snake_world.size() {
			for y in 0..self.snake_world.size() {
				let coord = Coord::new_usize(x, y);
				let cell = self.snake_world.get_cell(coord).copied();
				let rect = egui::Rect::from_min_size(
					rect.min + egui::vec2(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE),
					egui::vec2(CELL_SIZE, CELL_SIZE),
				);
				match cell {
					Some(snake::Cell::Empty) => {
						mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 0, 0));
					}
					Some(snake::Cell::Snake(iter)) => {
						let snake_precentage = iter as f32 / self.snake_world.snake_length() as f32;
						// let hue = snake_precentage * 360.0;

						let hsv =
							palette::Hsv::new(120.0, 1.0 - (1.0 - snake_precentage) * 0.8, 0.8);
						let rgb = palette::rgb::Rgb::from_color_unclamped(hsv);
						mesh.add_colored_rect(
							rect,
							egui::Color32::from_rgb(
								(rgb.red * 255.0) as u8,
								(rgb.green * 255.0) as u8,
								(rgb.blue * 255.0) as u8,
							),
						);
					}
					Some(snake::Cell::Food) => {
						mesh.add_colored_rect(rect, egui::Color32::from_rgb(255, 0, 0));
					}

					None => unreachable!(),
				}
			}
		}

		painter.add(egui::Shape::Mesh(mesh));

		// {
		// 	// Draw snake path
		// 	let mut iter = snake_path.iter_offsets();

		// 	let head = self.snake_world.snake_head_coord();

		// 	// Unwrap is safe here because the offsets iterator always starts with zero
		// 	let mut prev = iter.next().unwrap();

		// 	while let Some(offset) = iter.next() {
		// 		let start = head + prev;
		// 		let end = head + offset;

		// 		let half_cell = egui::vec2(CELL_SIZE / 2.0, CELL_SIZE / 2.0);

		// 		let start = rect.min
		// 			+ egui::vec2(start.x as f32 * CELL_SIZE, start.y as f32 * CELL_SIZE)
		// 			+ half_cell;
		// 		let end = rect.min
		// 			+ egui::vec2(end.x as f32 * CELL_SIZE, end.y as f32 * CELL_SIZE)
		// 			+ half_cell;

		// 		painter.add(egui::Shape::line_segment(
		// 			[start, end],
		// 			egui::Color32::from_rgb(0, 255, 0),
		// 		));

		// 		prev = offset;
		// 	}
		// }

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

		render_path(
			&snake_path,
			self.snake_world.snake_head_coord(),
			egui::Color32::from_rgb(0, 255 / 2, 0),
		);

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
