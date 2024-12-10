use regex::Regex;

fn part1(input: &str) {
    let re = Regex::new(r"mul\((?<args>\d{1,3},\d{1,3})\)").unwrap();
    let sum = re.captures_iter(input).fold(0, |acc, cap| {
        let (a, b) = cap["args"].split_once(',').unwrap();
        acc + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    });
    println!("{}", sum);
}

fn part2(input: &str) {
    let re = Regex::new(r"(?<op>mul\((?<args>\d{1,3},\d{1,3})\)|do(n't)?\(\))").unwrap();
    let mut active = true;
    let sum = re.captures_iter(input).fold(0, |acc, cap| {
        let operation = cap["op"].split_once('(').unwrap().0;
        match operation {
            "do" => active = true,
            "don't" => active = false,
            "mul" if active => {
                let (a, b) = cap["args"].split_once(',').unwrap();
                return acc + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
            }
            _ => {}
        }
        acc
    });
    println!("{}", sum);
}
fn main() {
    let input_filename = "input";
    let input = std::fs::read_to_string(input_filename).expect("Unable to read file");
    part1(&input);
    part2(&input);
}
