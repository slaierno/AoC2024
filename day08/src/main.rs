// use common_libs::map::Map;
use common_libs::point::Point;
use std::collections::{HashMap, HashSet, LinkedList};

fn part1(antennas: &HashMap<char, LinkedList<Point>>, width: usize, height: usize) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for pos_list in antennas.values() {
        for pos in pos_list {
            for p in pos_list.iter().filter(|p| **p != *pos) {
                let dist = p - pos;
                let antinodes_pos = pos + dist + dist;
                if (0..height as isize).contains(&antinodes_pos.y)
                    && (0..width as isize).contains(&antinodes_pos.x)
                {
                    antinodes.insert(antinodes_pos);
                }
            }
        }
    }
    antinodes.len()
}
fn part2(antennas: &HashMap<char, LinkedList<Point>>, width: usize, height: usize) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for pos_list in antennas.values() {
        for pos in pos_list {
            for p in pos_list.iter().filter(|p| **p != *pos) {
                let dist = p - pos;
                let mut antinodes_pos = *p;
                while (0..height as isize).contains(&antinodes_pos.y)
                    && (0..width as isize).contains(&antinodes_pos.x)
                {
                    antinodes.insert(antinodes_pos);
                    antinodes_pos = antinodes_pos + dist;
                }
            }
        }
    }
    antinodes.len()
}
fn main() {
    let _input_filename = "demo";
    let _input_filename = "input";
    let input: Vec<Vec<char>> = std::fs::read_to_string(_input_filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let antennas: HashMap<char, LinkedList<Point>> = {
        let mut ret = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.iter().enumerate().filter(|(_, c)| **c != '.') {
                let p = Point::from_usize(x, y);
                if !ret.contains_key(c) {
                    ret.insert(*c, LinkedList::new());
                }
                ret.get_mut(c).unwrap().push_back(p);
            }
        }
        ret
    };
    assert_eq!(222, part1(&antennas, input[0].len(), input.len()));
    assert_eq!(884, part2(&antennas, input[0].len(), input.len()));
}
