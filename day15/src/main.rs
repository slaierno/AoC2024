use common_libs::map::{Map, Point};
mod direction;
mod tile;
use direction::Direction;
use itertools::Itertools;
use tile::Tile;

trait Day15Map {
    fn widen(&self) -> Map<Tile>;
    fn get_gps_score(&self) -> isize;
    fn execute_instructions(&self, inst_list: Vec<Direction>) -> Map<Tile>;
}

impl Day15Map for Map<Tile> {
    fn widen(&self) -> Map<Tile> {
        Map {
            map: self
                .iter_rows()
                .map(|r| r.iter().flat_map(Tile::double).collect_vec())
                .collect_vec(),
        }
    }
    fn get_gps_score(&self) -> isize {
        self.enumerate()
            .filter(|(_, c)| **c == Tile::Box || **c == Tile::LeftBox)
            .fold(0, |acc, (p, _)| acc + 100 * p.y + p.x)
    }
    fn execute_instructions(&self, inst_list: Vec<Direction>) -> Map<Tile> {
        let mut map = self.clone();
        let start_robot_pos = map.position(Tile::Robot).unwrap(); // TODO get this from process_input, maybe?
        inst_list.iter().fold(start_robot_pos, |robot_pos, inst| {
            robot_pos
                + push(&mut map, robot_pos, *inst)
                    .then_some(inst.to_point())
                    .unwrap_or_default()
        });
        map
    }
}

fn process_input(input_str: &String) -> (Map<Tile>, Vec<Direction>) {
    let (map_input, instruction_input) = input_str.split_once("\n\n").unwrap();
    let instruction_input = instruction_input
        .lines()
        .flat_map(|l| l.chars())
        .map(Direction::from_char)
        .collect_vec();
    let m = Map {
        map: map_input
            .lines()
            .map(|l| l.chars().map(Tile::from_char).collect())
            .collect(),
    };
    (m, instruction_input)
}

fn can_move(m: &Map<Tile>, pos: Point, dir: Direction) -> bool {
    let object_pos = pos + dir;
    let object = m[object_pos];
    match object {
        Tile::Wall => false,
        Tile::Empty => true,
        Tile::Box | Tile::Robot => can_move(m, object_pos, dir),
        Tile::LeftBox | Tile::RightBox => {
            can_move(m, object_pos, dir)
                && (dir.is_horizontal()
                    || can_move(m, object_pos + object.opposite_box_side_direction(), dir))
        }
    }
}
fn push(m: &mut Map<Tile>, pos: Point, dir: Direction) -> bool {
    can_move(m, pos, dir)
        .then(|| push_action(m, pos, dir))
        .is_some()
}

fn push_action(m: &mut Map<Tile>, pos: Point, dir: Direction) {
    let object_pos = pos + dir;
    let object = m[object_pos];
    match object {
        Tile::Empty => (),
        Tile::Box | Tile::Robot => push_action(m, object_pos, dir),
        Tile::LeftBox | Tile::RightBox => {
            push_action(m, object_pos, dir);
            if dir.is_vertical() && object != m[pos] {
                push_action(m, object_pos + object.opposite_box_side_direction(), dir)
            }
        }
        Tile::Wall => panic!("Trying to push a wall!"),
    };
    m.swap(pos, object_pos);
}

fn part1(input_str: &String) -> isize {
    let (map, inst_list) = process_input(&input_str);
    map.execute_instructions(inst_list).get_gps_score()
}

fn part2(input_str: &String) -> isize {
    let (map, inst_list) = process_input(&input_str);
    map.widen().execute_instructions(inst_list).get_gps_score()
}

#[test]
fn demo() {
    let input_str = "\
                            ##########\n\
                            #..O..O.O#\n\
                            #......O.#\n\
                            #.OO..O.O#\n\
                            #..O@..O.#\n\
                            #O#..O...#\n\
                            #O..O..O.#\n\
                            #.OO.O.OO#\n\
                            #....O...#\n\
                            ##########\n\
                            \n\
                            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
                            ".to_string();
    assert_eq!(10092, part1(&input_str));
    assert_eq!(9021, part2(&input_str));
}

fn main() {
    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    assert_eq!(1552463, part1(&input_str));
    assert_eq!(1554058, part2(&input_str));
}
