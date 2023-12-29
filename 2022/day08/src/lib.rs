use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn cell(&self, coord: GridCoord) -> Option<&T> {
        // self.in_bounds(coord)
        //     .then_some(&self.data[coord.y * self.width + coord.x])
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    pub fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        // self.in_bounds(coord)
        //     .then_some(&mut self.data[coord.y * self.width + coord.x])

        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
