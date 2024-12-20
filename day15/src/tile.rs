use common_libs::{
    map::Point,
    point::{LX_VEC, RX_VEC},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
    LeftBox,
    RightBox,
}

impl Tile {
    pub fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            '[' => Tile::LeftBox,
            ']' => Tile::RightBox,
            _ => panic!("Invalid tile"),
        }
    }
    pub fn opposite_box_side_direction(&self) -> Point {
        match self {
            Tile::LeftBox => RX_VEC,
            Tile::RightBox => LX_VEC,
            _ => panic!("Invalid tile"),
        }
    }

    pub fn double(&self) -> [Tile; 2] {
        match self {
            Tile::Box => [Tile::LeftBox, Tile::RightBox],
            Tile::Robot => [Tile::Robot, Tile::Empty],
            _ => [self.clone(), self.clone()],
        }
    }
}
