fn concat(a: u64, b: u64) -> u64 {
    let mut pow = 10;
    while b >= pow {
        pow *= 10;
    }
    a * pow + b
}

fn is_valid((result, operands): &(u64, Vec<u64>), operators: &[fn(u64, u64) -> u64]) -> bool {
    operands[1..]
        .iter()
        .fold(vec![operands[0]], |acc: Vec<u64>, x| {
            acc.iter()
                .flat_map(|r| operators.iter().map(|f| f(*r, *x)).collect::<Vec<u64>>())
                .collect()
        })
        .contains(result)
}

fn count_valids(input: &Vec<(u64, Vec<u64>)>, operators: &[fn(u64, u64) -> u64]) -> u64 {
    input
        .iter()
        .filter(|x| is_valid(x, operators))
        .map(|x| x.0)
        .sum()
}

fn main() {
    let _input_filename = "demo";
    let _input_filename = "input";
    let input: Vec<(u64, Vec<u64>)> = std::fs::read_to_string(_input_filename)
        .unwrap()
        .lines()
        .map(|line| {
            let (result, operands) = line.split_once(":").unwrap();
            let result = result.parse::<u64>().unwrap();
            let operands: Vec<u64> = operands
                .split_whitespace()
                .map(|operand| operand.parse::<u64>().unwrap())
                .collect();
            (result, operands)
        })
        .collect();

    let part1_operators = vec![|a, b| a + b, |a, b| a * b];
    let part2_operators = vec![|a, b| a + b, |a, b| a * b, concat];
    println!("{}", count_valids(&input, &part1_operators)); // 1399219271639
    println!("{}", count_valids(&input, &part2_operators)); // 275791737999003
}
