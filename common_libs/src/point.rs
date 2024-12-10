use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub const DN_VEC: Point = Point { x: 0, y: 1 };
pub const LX_VEC: Point = Point { x: -1, y: 0 };
pub const UP_VEC: Point = Point { x: 0, y: -1 };
pub const RX_VEC: Point = Point { x: 1, y: 0 };

impl Point {
    pub fn from_usize(x: usize, y: usize) -> Point {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn rotate_cw(self) -> Point {
        match self {
            DN_VEC => LX_VEC,
            LX_VEC => UP_VEC,
            UP_VEC => RX_VEC,
            RX_VEC => DN_VEC,
            _ => panic!("Invalid direction"),
        }
    }
    pub fn dist_vec(self, other: &Point) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

impl<'a, B> std::ops::Add<B> for &'a Point
where
    B: Borrow<Point>,
{
    type Output = Point;

    fn add(self, rhs: B) -> Self::Output {
        Point {
            x: self.x + rhs.borrow().x,
            y: self.y + rhs.borrow().y,
        }
    }
}

impl<B> std::ops::Add<B> for Point
where
    B: Borrow<Point>,
{
    type Output = Point;

    fn add(self, rhs: B) -> Self::Output {
        &self + rhs
    }
}
