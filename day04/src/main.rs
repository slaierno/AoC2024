use itertools::Itertools;

fn part1(lines: &Vec<Vec<u8>>) -> usize {
    let width = lines[0].len();
    let height = lines.len();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if lines[y][x] == b'X' {
                let can_move_right = (x + 3) < width;
                let can_move_left = x >= 3;
                let can_move_up = y >= 3;
                let can_move_down = (y + 3) < height;
                if can_move_right
                    && (0..4).map(|i| lines[y][x + i] as char).collect::<String>() == "XMAS"
                {
                    count += 1;
                }
                if can_move_right
                    && can_move_down
                    && (0..4)
                        .map(|i| lines[y + i][x + i] as char)
                        .collect::<String>()
                        == "XMAS"
                {
                    count += 1;
                }
                if can_move_down
                    && (0..4).map(|i| lines[y + i][x] as char).collect::<String>() == "XMAS"
                {
                    count += 1;
                }
                if can_move_left
                    && can_move_down
                    && (0..4)
                        .map(|i| lines[y + i][x - i] as char)
                        .collect::<String>()
                        == "XMAS"
                {
                    count += 1;
                }
                if can_move_left
                    && (0..4).map(|i| lines[y][x - i] as char).collect::<String>() == "XMAS"
                {
                    count += 1;
                }
                if can_move_left
                    && can_move_up
                    && (0..4)
                        .map(|i| lines[y - i][x - i] as char)
                        .collect::<String>()
                        == "XMAS"
                {
                    count += 1;
                }
                if can_move_up
                    && (0..4).map(|i| lines[y - i][x] as char).collect::<String>() == "XMAS"
                {
                    count += 1;
                }
                if can_move_up
                    && can_move_right
                    && (0..4)
                        .map(|i| lines[y - i][x + i] as char)
                        .collect::<String>()
                        == "XMAS"
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(lines: &Vec<Vec<u8>>) -> usize {
    let width = lines[0].len();
    let height = lines.len();
    (1..height - 1)
        .cartesian_product(1..width - 1)
        .filter(|(x, y)| {
            [b"SAM", b"MAS"].contains(&&[lines[y - 1][x - 1], lines[*y][*x], lines[y + 1][x + 1]])
                && [b"SAM", b"MAS"].contains(&&[
                    lines[y + 1][x - 1],
                    lines[*y][*x],
                    lines[y - 1][x + 1],
                ])
        })
        .count()
}

fn main() {
    let input_filename = "input";
    let input = std::fs::read_to_string(input_filename).expect("Unable to read file");
    let lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    println!("-------");
    println!("PART 1:");
    println!("-------");
    println!("{}", part1(&lines)); // 2462

    println!("-------");
    println!("PART 2:");
    println!("-------");
    println!("{}", part2(&lines)); // 1877
}
