#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl std::ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.dx;
        self.y += rhs.dy;
    }
}

impl std::ops::Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl std::ops::Sub<Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.dx,
            y: self.y - rhs.dy,
        }
    }
}

impl std::ops::SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.dx;
        self.y -= rhs.dy;
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Vector {
    pub dx: i64,
    pub dy: i64,
}

impl Vector {
    pub const fn new(dx: i64, dy: i64) -> Self {
        Vector { dx, dy }
    }
}
