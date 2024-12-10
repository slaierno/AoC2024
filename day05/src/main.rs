use std::collections::{HashMap, HashSet};
type Rules = HashMap<i32, HashSet<i32>>;
fn get_rules(rules_str: &str) -> (Rules, Rules) {
    let mut forward_rules: Rules = HashMap::new();
    let mut backward_rules: Rules = HashMap::new();
    for rule in rules_str.lines() {
        let (before, after) = rule.split_once("|").unwrap();
        let before_val = before.parse::<i32>().unwrap();
        let after_val = after.parse::<i32>().unwrap();
        forward_rules
            .entry(before_val)
            .or_insert(HashSet::new())
            .insert(after_val);
        backward_rules
            .entry(after_val)
            .or_insert(HashSet::new())
            .insert(before_val);
    }
    (forward_rules, backward_rules)
}

fn get_pages_list(pages_str: &str) -> Vec<Vec<i32>> {
    pages_str
        .lines()
        .map(|l| l.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}

fn pages_statisfy_rules(rules: &(Rules, Rules), pages: &Vec<i32>) -> bool {
    for i in 0..pages.len() {
        let back: Vec<i32> = pages[..i].to_vec();
        let el: i32 = pages[i];
        let front: Vec<i32> = pages[i + 1..].to_vec();
        let (front_rules, back_rules) = rules;
        if back_rules.contains_key(&el) && front.into_iter().any(|e| back_rules[&el].contains(&e)) {
            return false;
        }
        if front_rules.contains_key(&el) && back.into_iter().any(|e| front_rules[&el].contains(&e))
        {
            return false;
        }
    }
    true
}

fn reorder_pages<'a>(rules: &(Rules, Rules), pages: &Vec<i32>) -> Vec<i32> {
    let (front_rules, back_rules) = rules;
    let mut ordered_pages: Vec<i32> = pages.clone();
    ordered_pages.sort_by(|a, b| {
        if front_rules.contains_key(a) && front_rules[a].contains(b) {
            std::cmp::Ordering::Less
        } else if back_rules.contains_key(a) && back_rules[a].contains(b) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    ordered_pages
}
fn extract_middle(l: &Vec<i32>) -> i32 {
    let middle_index = l.len() / 2;
    l[middle_index]
}

fn part1(rules: &(Rules, Rules), pages_list: &Vec<Vec<i32>>) -> i32 {
    pages_list
        .into_iter()
        .filter(|pages| pages_statisfy_rules(rules, pages))
        .map(extract_middle)
        .sum()
}

fn part2(rules: &(Rules, Rules), pages_list: &Vec<Vec<i32>>) -> i32 {
    pages_list
        .into_iter()
        .filter(|pages| !pages_statisfy_rules(rules, pages))
        .map(|pages| reorder_pages(rules, pages))
        .map(|pages| extract_middle(&pages))
        .sum()
}
fn main() {
    let input_filename = "input";
    let input = std::fs::read_to_string(input_filename).expect("Unable to read file");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let (rules, pages_list) = (get_rules(input[0]), get_pages_list(input[1]));

    println!("-------");
    println!("PART 1:");
    println!("-------");
    println!("{}", part1(&rules, &pages_list));

    println!("-------");
    println!("PART 2:");
    println!("-------");
    println!("{}", part2(&rules, &pages_list));
}
