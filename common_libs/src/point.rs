use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub const ZERO: Point = Point { x: 0, y: 0 };
pub const DN_VEC: Point = Point { x: 0, y: 1 };
pub const LX_VEC: Point = Point { x: -1, y: 0 };
pub const UP_VEC: Point = Point { x: 0, y: -1 };
pub const RX_VEC: Point = Point { x: 1, y: 0 };

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
    pub fn from_usize(x: usize, y: usize) -> Point {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn from_tuple((x, y): (usize, usize)) -> Point {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn from_string(input_str: &str) -> Point {
        let (x, y) = input_str.trim().split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
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
    pub fn ortho_neighbours(&self) -> impl Iterator<Item = Point> {
        [DN_VEC, UP_VEC, LX_VEC, RX_VEC]
            .map(|p| p + self)
            .into_iter()
    }

    pub fn north(&self) -> Point {
        self + UP_VEC
    }
    pub fn south(&self) -> Point {
        self + DN_VEC
    }
    pub fn west(&self) -> Point {
        self + LX_VEC
    }
    pub fn east(&self) -> Point {
        self + RX_VEC
    }
}

macro_rules! impl_add_assigns {
    ($t:ty) => {
        impl<B> std::ops::AddAssign<B> for $t
        where
            B: Borrow<Point>,
        {
            fn add_assign(&mut self, rhs: B) {
                self.x += rhs.borrow().x;
                self.y += rhs.borrow().y;
            }
        }
    };
}
macro_rules! impl_adds {
    ($lt:lifetime, $t:ty) => {
        impl<$lt, B> std::ops::Add<B> for $t
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
        impl<$lt, B> std::ops::Sub<B> for $t
        where
            B: Borrow<Point>,
        {
            type Output = Point;

            fn sub(self, rhs: B) -> Self::Output {
                Point {
                    x: self.x - rhs.borrow().x,
                    y: self.y - rhs.borrow().y,
                }
            }
        }
    };
}
macro_rules! impl_mul_assigns {
    ($t:ty) => {
        impl std::ops::MulAssign<isize> for $t {
            fn mul_assign(&mut self, rhs: isize) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }
    };
}
macro_rules! impl_muls {
    ($lt:lifetime, $lhs_t:ty, $rhs_t:ty) => {
        impl<$lt> std::ops::Mul<$rhs_t> for $lhs_t {
            type Output = Point;

            fn mul(self, rhs: $rhs_t) -> Self::Output {
                Point {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
        impl<$lt> std::ops::Mul<$lhs_t> for $rhs_t {
            type Output = Point;

            fn mul(self, rhs: $lhs_t) -> Self::Output {
                Point {
                    x: self * rhs.x,
                    y: self * rhs.y,
                }
            }
        }
    };
}

impl_add_assigns!(Point);
impl_add_assigns!(&mut Point);
impl_adds!('a, Point);
impl_adds!('a, &'a Point);

impl_mul_assigns!(Point);
impl_mul_assigns!(&mut Point);
impl_muls!('a, Point, isize);
impl_muls!('a, &'a Point, isize);
impl_muls!('a, Point, &isize);
impl_muls!('a, &'a Point, &isize);

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}
