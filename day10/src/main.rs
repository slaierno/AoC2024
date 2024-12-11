use std::collections::HashSet;

use common_libs::map::Map;
use common_libs::point::Point;
use itertools::Itertools;
type Height = u32;
fn get_score(
    trailing_map: &Map<Height>,
    curr_pos: &Point,
    visited_points: &mut HashSet<Point>,
) -> u32 {
    let curr_height = trailing_map[curr_pos];
    visited_points.insert(*curr_pos);
    match curr_height {
        9 => 1,
        _ => curr_pos
            .ortho_neighbours()
            .filter_map(|p| {
                if trailing_map.is_pos_inside(&p)
                    && trailing_map[p] == trailing_map[curr_pos] + 1
                    && !visited_points.contains(&p)
                {
                    Some(get_score(trailing_map, &p, visited_points))
                } else {
                    None
                }
            })
            .sum(),
    }
}

fn get_rating(trailing_map: &Map<Height>, curr_pos: &Point) -> u32 {
    let curr_height = trailing_map[curr_pos];
    match curr_height {
        9 => 1,
        _ => curr_pos
            .ortho_neighbours()
            .filter_map(|p| {
                if trailing_map.is_pos_inside(&p) && trailing_map[p] == trailing_map[curr_pos] + 1 {
                    Some(get_rating(trailing_map, &p))
                } else {
                    None
                }
            })
            .sum(),
    }
}

fn process_input(input_str: &String) -> Vec<Vec<Height>> {
    input_str
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn part1(input_str: &String) -> u32 {
    let trailing_map = Map {
        map: process_input(&input_str),
    };

    trailing_map
        .find_all_positions(&0)
        .map(|p| get_score(&trailing_map, &p, &mut HashSet::<Point>::new()))
        .sum()
}

fn part2(input_str: &String) -> u32 {
    let trailing_map = Map {
        map: process_input(&input_str),
    };

    trailing_map
        .find_all_positions(&0)
        .map(|p| get_rating(&trailing_map, &p))
        .sum()
}
fn test() {
    let input_str = "89010123\n\
                             78121874\n\
                             87430965\n\
                             96549874\n\
                             45678903\n\
                             32019012\n\
                             01329801\n\
                             10456732"
        .to_string();

    assert_eq!(36, part1(&input_str));
    assert_eq!(81, part2(&input_str));
}

fn main() {
    test();

    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    println!("{}", part1(&input_str));
    println!("{}", part2(&input_str));
}
