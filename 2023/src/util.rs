#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> Grid<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, point: &Point) -> &T {
        &self.data[point.row + point.col * self.cols]
    }

    pub fn set(&mut self, point: &Point, value: T) {
        self.data[point.row + point.col * self.cols] = value;
    }
}

impl<T> Grid<T> where T: Default {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data = Vec::new();

        data.resize_with(rows * cols, T::default);

        Grid {
            rows,
            cols,
            data,
        }
    }
}

impl<T> Grid<T> where T: Clone {
    pub fn new_fill(rows: usize, cols: usize, value: T) -> Self {
        let mut data = Vec::new();

        data.resize(rows * cols, value);

        Grid {
            rows,
            cols,
            data,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn up(&self, distance: usize) -> Self {
        Point {
            row: self.row - distance,
            col: self.col,
        }
    }

    pub fn down(&self, distance: usize) -> Self {
        Point {
            row: self.row + distance,
            col: self.col,
        }
    }

    pub fn left(&self, distance: usize) -> Self {
        Point {
            row: self.row,
            col: self.col - distance,
        }
    }

    pub fn right(&self, distance: usize) -> Self {
        Point {
            row: self.row,
            col: self.col + distance,
        }
    }
}
