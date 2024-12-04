use itertools::Itertools;

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

    pub fn get(&self, point: &Point) -> Option<&T> {
        if point.row < 0 || point.row >= self.rows as i32 {
            return None;
        }

        if point.col < 0 || point.col >= self.cols as i32 {
            return None;
        }

        Some(&self.data[point.row as usize * self.cols + point.col as usize])
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.rows as i32).cartesian_product(0..self.cols as i32).map(|(row, col)| Point { row, col })
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
    pub row: i32,
    pub col: i32,
}
impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub const VALUES: [Self; 8] = [
        Self::Up,
        Self::UpRight,
        Self::Right,
        Self::DownRight,
        Self::Down,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
    ];
}

impl std::ops::Mul<i32> for Direction {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Direction::Up => Vector {
                row: -rhs,
                col: 0,
            },
            Direction::UpRight => Vector {
                row: -rhs,
                col: rhs,
            },
            Direction::Right => Vector {
                row: 0,
                col: rhs,
            },
            Direction::DownRight => Vector {
                row: rhs,
                col: rhs,
            },
            Direction::Down => Vector {
                row: rhs,
                col: 0,
            },
            Direction::DownLeft => Vector {
                row: rhs,
                col: -rhs,
            },
            Direction::Left => Vector {
                row: 0,
                col: -rhs,
            },
            Direction::UpLeft => Vector {
                row: -rhs,
                col: -rhs,
            },
        }
    }
}

impl std::ops::Mul<i32> for &Direction {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Direction::Up => Vector {
                row: -rhs,
                col: 0,
            },
            Direction::UpRight => Vector {
                row: -rhs,
                col: rhs,
            },
            Direction::Right => Vector {
                row: 0,
                col: rhs,
            },
            Direction::DownRight => Vector {
                row: rhs,
                col: rhs,
            },
            Direction::Down => Vector {
                row: rhs,
                col: 0,
            },
            Direction::DownLeft => Vector {
                row: rhs,
                col: -rhs,
            },
            Direction::Left => Vector {
                row: 0,
                col: -rhs,
            },
            Direction::UpLeft => Vector {
                row: -rhs,
                col: -rhs,
            },
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Vector {
    pub row: i32,
    pub col: i32,
}
