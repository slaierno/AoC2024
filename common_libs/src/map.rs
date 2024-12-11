pub use crate::point::Point;
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

    // fn is_obstacle(&self, pos: &Point) -> bool {
    //     self.is_pos_inside(pos) && self[pos] == OBSTACLE
    // }
    // fn clone_with_new_obstacle(&self, pos: &Point) -> Map {
    //     let mut new_map = self.clone();
    //     new_map[pos] = OBSTACLE;
    //     new_map
    // }

    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.map.iter()
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
