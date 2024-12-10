// use std::collections::HashMap;
// use std::cmp::Ordering;

use itertools::Itertools;

fn is_safe(levels: &Vec<i32>) -> bool {
    let ordering = levels[0].cmp(&levels[1]);
    levels
        .windows(2)
        .all(|w| ordering == w[0].cmp(&w[1]) && (1..4).contains(&(w[1] - w[0]).abs()))
}

fn part1(reports: &Vec<Vec<i32>>) {
    println!("-------");
    println!("PART 1:");
    println!("-------");
    println!(
        "Number of safe reports: {}",
        reports.iter().filter(|&levels| is_safe(levels)).count()
    );
}

fn part2(reports: &Vec<Vec<i32>>) {
    println!("-------");
    println!("PART 2:");
    println!("-------");
    println!(
        "Number of safe reports: {}",
        reports
            .iter()
            .filter(|&report| (0..report.len())
                .any(|i| is_safe(&[&report[..i], &report[i + 1..]].concat())))
            .count()
    );
}

fn main() {
    let input_filename = "input";
    let reports = std::fs::read_to_string(input_filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    part1(&reports);
    part2(&reports);
}
