use std::collections::HashMap;

use itertools::Itertools;
fn blink(n: u64) -> Vec<u64> {
    match n {
        0 => vec![1u64],
        _ => {
            let n_of_digits = n.ilog10() + 1;
            if n_of_digits % 2 == 0 {
                vec![
                    n / 10u64.pow(n_of_digits / 2),
                    n % 10u64.pow(n_of_digits / 2),
                ]
            } else {
                vec![n * 2024]
            }
        }
    }
}

fn blink_n(number: u64, times: u64) -> Vec<u64> {
    match times {
        1 => blink(number),
        _ => blink(number)
            .iter()
            .flat_map(|x| blink_n(*x, times - 1))
            .collect(),
    }
}

fn blink_n_count_impl(number: u64, times: u64, memo: &mut HashMap<(u64, u64), usize>) -> usize {
    if memo.contains_key(&(number, times)) {
        return memo.get(&(number, times)).unwrap().clone();
    }
    let ret = match times {
        1 => blink(number).len(),
        _ => blink(number)
            .iter()
            .map(|x| blink_n_count_impl(*x, times - 1, memo))
            .sum::<usize>(),
    };
    memo.insert((number, times), ret);
    ret
}

fn blink_n_count(number: u64, times: u64) -> usize {
    let mut memo = HashMap::new();
    blink_n_count_impl(number, times, &mut memo)
}

fn process_input(input_str: &String) -> Vec<u64> {
    input_str
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn test() {
    {
        let input_str = "0 1 10 99 999".to_string();
        let input = process_input(&input_str);
        assert_eq!(
            vec![1, 2024, 1, 0, 9, 9, 2021976],
            input.iter().flat_map(|x| blink(*x)).collect_vec()
        );
    }
    {
        let input_str = "125 17".to_string();
        let input = process_input(&input_str);
        let expected_str =
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2".to_string();
        let expected_result = process_input(&expected_str);
        assert_eq!(
            expected_result,
            input.iter().flat_map(|x| blink_n(*x, 6)).collect_vec()
        );
        assert_eq!(55312, input.iter().flat_map(|x| blink_n(*x, 25)).count());
    }
}
fn main() {
    test();

    let input_str = "6563348 67 395 0 6 4425 89567 739318".to_string();
    let input = process_input(&input_str);
    assert_eq!(
        184927,
        input.iter().map(|x| blink_n_count(*x, 25)).sum::<usize>()
    );
    assert_eq!(
        220357186726677,
        input.iter().map(|x| blink_n_count(*x, 75)).sum::<usize>()
    );
}
