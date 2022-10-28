use eframe::egui::{self, Response, Sense, Widget};

use crate::{
    snake::{self, SnakeWorld},
    Coord,
};

pub struct SnakeWorldViewer<'a> {
    snake_world: &'a SnakeWorld,
}

impl<'a> SnakeWorldViewer<'a> {
    pub fn new(snake_world: &'a SnakeWorld) -> Self {
        Self { snake_world }
    }
}

impl Widget for SnakeWorldViewer<'_> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click());

        let mut mesh = egui::Mesh::default();
        for x in 0..self.snake_world.size() {
            for y in 0..self.snake_world.size() {
                let coord = Coord::new_usize(x, y);
                let cell = self.snake_world.get_cell(coord).copied();
                let rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2(x as f32 * 10.0, y as f32 * 10.0),
                    egui::vec2(10.0, 10.0),
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

        response
    }
}
