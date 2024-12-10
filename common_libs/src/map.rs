pub use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Map {
    pub map: Vec<Vec<char>>,
}

impl Map {
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
}

impl std::ops::Index<&Point> for Map {
    type Output = char;

    fn index(&self, pos: &Point) -> &Self::Output {
        &self.map[pos.y as usize][pos.x as usize]
    }
}

impl std::ops::IndexMut<&Point> for Map {
    fn index_mut(&mut self, pos: &Point) -> &mut Self::Output {
        &mut self.map[pos.y as usize][pos.x as usize]
    }
}
