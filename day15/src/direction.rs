use common_libs::point::Point;
use common_libs::point::{DN_VEC, LX_VEC, RX_VEC, UP_VEC};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Up => UP_VEC,
            Direction::Down => DN_VEC,
            Direction::Left => LX_VEC,
            Direction::Right => RX_VEC,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            _ => false,
        }
    }
    pub fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    pub fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid instruction character"),
        }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.to_point()
    }
}

impl std::ops::Add<Point> for Direction {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        rhs + self.to_point()
    }
}
