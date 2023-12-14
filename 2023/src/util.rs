#[derive(Clone, Hash, PartialEq, Eq, Debug)]
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
        &self.data[point.row * self.cols + point.col]
    }

    pub fn set(&mut self, point: &Point, value: T) {
        self.data[point.row * self.cols + point.col] = value;
    }

    pub fn get_row(&self, index: usize) -> Vec<&T> {
        if index >= self.rows {
            panic!("index >= self.rows");
        }

        (0..self.cols).map(|col| self.get(&Point { row: index, col })).collect()
    }

    pub fn get_col(&self, index: usize) -> Vec<&T> {
        if index >= self.cols {
            panic!("index >= self.cols");
        }

        (0..self.rows).map(|row| self.get(&Point { row, col: index })).collect()
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

impl<T> From<Vec<Vec<T>>> for Grid<T> where T: Clone {
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        let cols = value[0].len();
        let data = value.concat();

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
