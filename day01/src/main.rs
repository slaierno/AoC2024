use std::collections::HashMap;

fn part1(lines: &Vec<&str>) {
    println!("-------");
    println!("PART 1:");
    println!("-------");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        list1.push(numbers.next().unwrap().parse::<i32>().unwrap());
        list2.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut differences = Vec::new();
    for i in 0..list1.len() {
        differences.push((list1[i] - list2[i]).abs());
    }
    println!("Sum of differences: {}", differences.iter().sum::<i32>());
}

fn part2(lines: &Vec<&str>) {
    println!("-------");
    println!("PART 2:");
    println!("-------");
    let mut left_list = Vec::new();
    let mut right_map = HashMap::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        let (left, right) = (
            numbers.next().unwrap().parse::<i32>().unwrap(),
            numbers.next().unwrap().parse::<i32>().unwrap(),
        );
        left_list.push(left);
        match right_map.get(&right) {
            Some(n) => {
                right_map.insert(right, n + 1);
            }
            None => {
                right_map.insert(right, 1);
            }
        }
    }

    println!(
        "Similarity score: {}",
        left_list
            .iter()
            .map(|x| x * right_map.get(&x).unwrap_or(&0))
            .sum::<i32>()
    );
}

fn main() {
    // let input_filename = "demo.txt";
    let input_filename = "input";
    let input = std::fs::read_to_string(input_filename).expect("Unable to read file");
    let lines: Vec<&str> = input.lines().collect();

    part1(&lines);
    part2(&lines);
}
