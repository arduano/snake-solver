#![allow(dead_code)]

use crate::Coord;

pub struct Array2D<T> {
    size: usize,
    data: Box<[T]>,
}

impl<T> Array2D<T> {
    fn coord_to_index(&self, coord: Coord) -> Option<usize> {
        if !self.is_in_bounds(coord) {
            return None;
        } else {
            Some(coord.y as usize * self.size + coord.x as usize)
        }
    }

    fn coord_to_index_or_panic(&self, coord: Coord) -> usize {
        self.coord_to_index(coord).expect("Coord out of bounds")
    }

    pub fn new(size: usize, default: T) -> Self
    where
        T: Clone,
    {
        let data = vec![default; size * size].into_boxed_slice();
        Self { size, data }
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        Some(&self.data[self.coord_to_index(coord)?])
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        Some(&mut self.data[self.coord_to_index(coord)?])
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        self.data[self.coord_to_index_or_panic(coord)] = value;
    }

    pub fn is_in_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.size as i32 && coord.y < self.size as i32
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn count(&self) -> usize {
        self.size * self.size
    }
}

impl<T> std::ops::Index<Coord> for Array2D<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.data[self.coord_to_index_or_panic(coord)]
    }
}

impl<T> std::ops::IndexMut<Coord> for Array2D<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.data[self.coord_to_index_or_panic(coord)]
    }
}
