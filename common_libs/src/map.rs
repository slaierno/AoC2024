pub use crate::point::Point;
use itertools::Itertools;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Map<T = char> {
    pub map: Vec<Vec<T>>,
}

impl<T> Map<T> {
    pub fn width(&self) -> usize {
        self.map[0].len()
    }
    pub fn height(&self) -> usize {
        self.map.len()
    }
    pub fn is_pos_inside(&self, pos: &Point) -> bool {
        pos.x >= 0 && pos.x < self.width() as isize && pos.y >= 0 && pos.y < self.height() as isize
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.map.iter()
    }
    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.map.iter_mut()
    }

    pub fn get_all_positions_cr(&self) -> impl Iterator<Item = Point> {
        (0..self.width())
            .cartesian_product(0..self.height())
            .map(Point::from_tuple)
    }
    pub fn get_all_positions(&self) -> impl Iterator<Item = Point> {
        self.get_all_positions_cr()
    }
    pub fn get_all_positions_rc(&self) -> impl Iterator<Item = Point> {
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(|(y, x)| Point::from_usize(x, y))
    }

    pub fn get_or(&self, pos: &Point, default: T) -> T
    where
        T: Clone + Copy,
    {
        if self.is_pos_inside(pos) {
            self.map[pos.y as usize][pos.x as usize]
        } else {
            default
        }
    }

    pub fn position(&self, value: T) -> Option<Point>
    where
        T: PartialEq,
    {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.map[i][j] == value {
                    return Some(Point::from_usize(j, i));
                }
            }
        }
        None
    }

    pub fn swap(&mut self, pos1: Point, pos2: Point)
    where
        T: Clone + Copy,
    {
        let temp = self[pos1];
        self[pos1] = self[pos2];
        self[pos2] = temp;
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Point, &T)> {
        self.get_all_positions().map(move |pos| (pos, &self[pos]))
    }
}

impl Map<char> {
    pub fn from_str(input_str: &str) -> Map<char> {
        Map {
            map: input_str
                .lines()
                .map(|l| l.chars().collect_vec())
                .collect_vec(),
        }
    }

    pub fn dump(&self) -> String {
        self.map
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> Map<T>
where
    T: Clone,
{
    pub fn from_size_value(width: usize, height: usize, value: T) -> Map<T> {
        Map {
            map: vec![vec![value; width]; height],
        }
    }
    pub fn from_size_default(width: usize, height: usize) -> Map<T>
    where
        T: Default,
    {
        Self::from_size_value(width, height, T::default())
    }
}

impl<'a, T> Map<T>
where
    T: std::cmp::PartialEq + 'a,
{
    pub fn find_all_positions<'b>(&'b self, value: &'a T) -> impl Iterator<Item = Point> + 'b
    where
        'a: 'b,
    {
        self.iter_rows().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == *value)
                .map(move |(x, _)| Point::from_usize(x, y))
        })
    }
}

impl<'a, BPoint, T> std::ops::Index<BPoint> for Map<T>
where
    BPoint: Borrow<Point>,
{
    type Output = T;

    fn index(&self, pos: BPoint) -> &Self::Output {
        &self.map[pos.borrow().y as usize][pos.borrow().x as usize]
    }
}

impl<'a, BPoint, T> std::ops::IndexMut<BPoint> for Map<T>
where
    BPoint: Borrow<Point>,
{
    fn index_mut(&mut self, pos: BPoint) -> &mut Self::Output {
        &mut self.map[pos.borrow().y as usize][pos.borrow().x as usize]
    }
}
