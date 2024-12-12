use std::collections::BTreeMap;

use common_libs::map::{Map, Point};
use itertools::Itertools;
use partitions::{partition_vec, partitions_count_expr};
use velcro::btree_map;

const UNASSIGNED_COMPONENT: i32 = -1;
fn process_input(input_str: &String) -> Map<char> {
    Map {
        map: input_str
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec(),
    }
}

fn build_component_map(garden_map: &Map<char>) -> Map<i32> {
    let mut component_map: Map<i32> = Map::from_size_value(
        garden_map.width(),
        garden_map.height(),
        UNASSIGNED_COMPONENT,
    );
    let mut next_component = 0;
    let mut equivalence_labels = partition_vec![];
    for curr_pos in garden_map.get_all_positions() {
        let connected_ps = curr_pos
            .ortho_neighbours()
            .filter(|n| garden_map.is_pos_inside(n))
            .filter(|n| {
                component_map[n] != UNASSIGNED_COMPONENT && garden_map[n] == garden_map[curr_pos]
            })
            .collect_vec();
        match connected_ps.len() {
            0 => {
                component_map[curr_pos] = next_component;
                equivalence_labels.push(next_component);
                next_component += 1;
            }
            1 => {
                component_map[curr_pos] = component_map[connected_ps.first().unwrap()];
            }
            _ => {
                let min_label = connected_ps
                    .iter()
                    .map(|p| component_map[*p])
                    .min()
                    .unwrap();
                for &neigh_pos in connected_ps.iter() {
                    let duplicate_label = component_map[neigh_pos] as usize;
                    equivalence_labels.union(min_label as usize, duplicate_label);
                    component_map[neigh_pos] = min_label;
                }
                component_map[curr_pos] = min_label;
            }
        }
    }
    for r in component_map.iter_rows_mut() {
        for el in r.iter_mut() {
            let min_label = equivalence_labels.set(*el as usize).min().unwrap().1;
            *el = *min_label;
        }
    }
    component_map
}

fn get_area_perimeter(component_map: &Map<i32>) -> (BTreeMap<i32, usize>, BTreeMap<i32, usize>) {
    let mut areas = btree_map![];
    let mut perimeters = btree_map![];
    for p in component_map.get_all_positions() {
        let component_id = component_map[p];

        let area = *areas.get(&component_id).unwrap_or(&0);
        areas.insert(component_id, area + 1);

        let perimeter_points = p
            .ortho_neighbours()
            .filter(|n| !component_map.is_pos_inside(n) || component_map[n] != component_id)
            .count();

        let perimeter = *perimeters.get(&component_id).unwrap_or(&0);
        perimeters.insert(component_id, perimeter + perimeter_points);
    }
    (areas, perimeters)
}
fn get_area_side(component_map: &Map<i32>) -> (BTreeMap<i32, usize>, BTreeMap<i32, usize>) {
    let mut areas = btree_map![];
    // let mut side_labels = partition_vec![];
    for p in component_map.get_all_positions_cr() {
        let component_id = component_map[p];

        let area = *areas.get(&component_id).unwrap_or(&0);
        areas.insert(component_id, area + 1);
    }
    let mut n_of_sides = btree_map![];
    for x in 0..component_map.width() {
        for y in 0..component_map.height() {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            let curr_component_id = component_map[p];
            if curr_component_id != last_component_id {
                last_component_id = curr_component_id;
                let sides = *n_of_sides.get(&curr_component_id).unwrap_or(&0);
                n_of_sides.insert(curr_component_id, sides + 1);
            }
        }
    }
    for y in 0..component_map.height() {
        let mut last_component_id = UNASSIGNED_COMPONENT;
        for x in 0..component_map.width() {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            let curr_component_id = component_map[p];
            if curr_component_id != last_component_id {
                last_component_id = curr_component_id;
                let sides = *n_of_sides.get(&curr_component_id).unwrap_or(&0);
                n_of_sides.insert(curr_component_id, sides + 1);
            }
        }
    }
    (areas, n_of_sides)
}

fn part1(input_str: &String) -> usize {
    let garden_map = process_input(&input_str);
    let component_map = build_component_map(&garden_map);
    let (areas, perimeters) = get_area_perimeter(&component_map);
    areas
        .iter()
        .zip(perimeters.iter())
        .map(|(a, p)| a.1 * p.1)
        .sum()
}
fn part2(input_str: &String) -> usize {
    let garden_map = process_input(&input_str);
    let component_map = build_component_map(&garden_map);
    let (areas, sides) = get_area_side(&component_map);
    areas.iter().zip(sides.iter()).map(|(a, p)| a.1 * p.1).sum()
}
fn test() {
    {
        let input_str = "AAAA\n\
                                 BBCD\n\
                                 BBCC\n\
                                 EEEC"
            .to_string();
        let input = process_input(&input_str);
        assert_eq!(
            input.map,
            [
                ['A', 'A', 'A', 'A'],
                ['B', 'B', 'C', 'D'],
                ['B', 'B', 'C', 'C'],
                ['E', 'E', 'E', 'C']
            ]
        );
        let component_map = build_component_map(&input);
        assert_eq!(
            component_map.map,
            [[0, 0, 0, 0], [1, 1, 3, 4], [1, 1, 3, 3], [2, 2, 2, 3]]
        );
        let (areas, perimeters) = get_area_perimeter(&component_map);
        assert_eq!(btree_map![0:4, 1:4, 2:3, 3:4, 4:1], areas);
        assert_eq!(btree_map![0:10,1:8, 2:8, 3:10, 4:4], perimeters);
        assert_eq!(140, part1(&input_str));
        assert_eq!(80, part2(&input_str));
    }
    {
        let demo_str = "OOOOO\n\
                                OXOXO\n\
                                OOOOO\n\
                                OXOXO\n\
                                OOOOO"
            .to_string();
        assert_eq!(772, part1(&demo_str));
        assert_eq!(436, part2(&demo_str));
    }
    {
        let demo_str = "EEEEE\n\
                                EXXXX\n\
                                EEEEE\n\
                                EXXXX\n\
                                EEEEE"
            .to_string();
        assert_eq!(236, part2(&demo_str));
    }
    {
        let demo_str = "AAAAAA\n\
                                AAABBA\n\
                                AAABBA\n\
                                ABBAAA\n\
                                ABBAAA\n\
                                AAAAAA"
            .to_string();
        assert_eq!(368, part2(&demo_str));
    }
    {
        let demo_str = "RRRRIICCFF\n\
                                RRRRIICCCF\n\
                                VVRRRCCFFF\n\
                                VVRCCCJFFF\n\
                                VVVVCJJCFE\n\
                                VVIVCCJJEE\n\
                                VVIIICJJEE\n\
                                MIIIIIJJEE\n\
                                MIIISIJEEE\n\
                                MMMISSJEEE"
            .to_string();
        assert_eq!(1930, part1(&demo_str));
        assert_eq!(1206, part2(&demo_str));
    }
}
fn main() {
    test();

    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    assert_eq!(1433460, part1(&input_str));
}
