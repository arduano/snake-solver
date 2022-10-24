#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

pub struct Array2D<T> {
    size: usize,
    data: Box<[T]>,
}

impl<T> Array2D<T> {
    fn coord_to_index(&self, coord: Coord) -> usize {
        coord.y * self.size + coord.x
    }

    pub fn new(size: usize, default: T) -> Self
    where
        T: Clone,
    {
        let data = vec![default; size * size].into_boxed_slice();
        Self { size, data }
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        if coord.x < self.size && coord.y < self.size {
            Some(&self.data[self.coord_to_index(coord)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if coord.x < self.size && coord.y < self.size {
            Some(&mut self.data[self.coord_to_index(coord)])
        } else {
            None
        }
    }

    pub fn set(&mut self, coord: Coord, value: T) -> Option<()> {
        if coord.x < self.size && coord.y < self.size {
            self.data[self.coord_to_index(coord)] = value;
            Some(())
        } else {
            None
        }
    }

    pub fn is_in_bounds(&self, coord: Coord) -> bool {
        coord.x < self.size && coord.y < self.size
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
        &self.data[self.coord_to_index(coord)]
    }
}

impl<T> std::ops::IndexMut<Coord> for Array2D<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.data[self.coord_to_index(coord)]
    }
}
