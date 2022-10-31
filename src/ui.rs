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
                    Some(snake::Cell::Snake(_)) => {
                        mesh.add_colored_rect(rect, egui::Color32::from_rgb(0, 255, 0));
                    }
                    Some(snake::Cell::Food) => {
                        mesh.add_colored_rect(rect, egui::Color32::from_rgb(255, 0, 0));
                    }

                    None => unreachable!(),
                }
            }
        }

        let painter = ui.painter();
        painter.add(egui::Shape::Mesh(mesh));

        if let Some(path) = self.overlay_path {
            let mut iter = path.iter_offsets();

            // Unwrap is safe here because the offsets iterator always starts with zero
            let mut prev = iter.next().unwrap();
            let head = self.snake_world.snake_head_coord();

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
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 0)),
                ));

                prev = offset;
            }
        }

        response
    }
}
