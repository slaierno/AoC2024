use common_libs::map::Map;
use common_libs::point::*;

use std::collections::HashSet;

const OBSTACLE: char = '#';
const ARROWS: [char; 4] = ['v', '^', '>', '<'];

trait MapExtension {
    fn is_obstacle(&self, pos: &Point) -> bool;
    fn clone_with_new_obstacle(&self, pos: &Point) -> Map;
}

impl MapExtension for Map {
    fn is_obstacle(&self, pos: &Point) -> bool {
        self.is_pos_inside(pos) && self[pos] == OBSTACLE
    }
    fn clone_with_new_obstacle(&self, pos: &Point) -> Map {
        let mut new_map = self.clone();
        new_map[pos] = OBSTACLE;
        new_map
    }
}

fn part1(guard_start_pos: &Point, guard_start_dir: &Point, map: &Map) -> HashSet<Point> {
    let mut guard_pos = *guard_start_pos;
    let mut guard_dir = *guard_start_dir;
    let mut visited: HashSet<Point> = HashSet::new();
    while map.is_pos_inside(&guard_pos) {
        visited.insert(guard_pos);
        let new_pos = guard_pos + guard_dir;
        if !map.is_obstacle(&new_pos) {
            guard_pos = new_pos;
        } else {
            guard_dir = guard_dir.rotate_cw();
        }
    }
    visited
}

fn has_loop(map: &Map, guard_start_pos: &Point, guard_start_dir: &Point) -> bool {
    let mut guard_pos = *guard_start_pos;
    let mut guard_dir = *guard_start_dir;
    let mut visited: HashSet<(Point, Point)> = HashSet::new();
    while map.is_pos_inside(&guard_pos) {
        visited.insert((guard_pos, guard_dir));
        let new_pos = guard_pos + guard_dir;
        if !map.is_obstacle(&new_pos) {
            guard_pos = new_pos;
        } else {
            guard_dir = guard_dir.rotate_cw();
        }
        if visited.contains(&(guard_pos, guard_dir)) {
            return true;
        }
    }
    false
}
fn part2(
    guard_start_pos: &Point,
    guard_start_dir: &Point,
    map: &Map,
    visited: &HashSet<Point>,
) -> usize {
    visited
        .into_iter()
        .filter(|pos| {
            *pos != guard_start_pos
                && has_loop(
                    &map.clone_with_new_obstacle(pos),
                    guard_start_pos,
                    guard_start_dir,
                )
        })
        .count()
}

fn main() {
    let input_filename = "input";
    let input = std::fs::read_to_string(input_filename).expect("Unable to read file");
    let flat_input = input.replace('\n', "");
    let map = Map {
        map: input
            .lines()
            .map(|l| l.replace(ARROWS, ".").chars().collect())
            .collect(),
    };
    let guard_start_abs_pos = flat_input.find(|c| ARROWS.contains(&c)).unwrap();
    let guard_start_pos = Point {
        x: (guard_start_abs_pos % map.width()) as isize,
        y: (guard_start_abs_pos / map.height()) as isize,
    };
    let guard_start_dir = match flat_input.chars().nth(guard_start_abs_pos).unwrap() {
        'v' => DN_VEC,
        '<' => LX_VEC,
        '^' => UP_VEC,
        '>' => RX_VEC,
        _ => panic!("Invalid guard direction"),
    };

    println!("-------");
    println!("PART 1:");
    println!("-------");
    let p1 = part1(&guard_start_pos, &guard_start_dir, &map);
    println!("{}", p1.len()); //5331
    println!("-------");
    println!("PART 2:");
    println!("-------");
    println!("{}", part2(&guard_start_pos, &guard_start_dir, &map, &p1)); //1812
}
