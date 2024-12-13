use itertools::Itertools;

struct ClawMachine {
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_prize: i64,
    y_prize: i64,
}

impl ClawMachine {
    fn line_to_values(button_line: &str, delimiter: &str) -> (i64, i64) {
        button_line
            .split_terminator(",")
            .map(|s| s.split_once(delimiter).unwrap().1.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap()
    }
    pub fn from_string(input_str: &str) -> ClawMachine {
        let (a_button, b_button, prize_info) = input_str.lines().collect_tuple().unwrap();
        let (x_a, y_a) = Self::line_to_values(&a_button, "+");
        let (x_b, y_b) = Self::line_to_values(&b_button, "+");
        let (x_prize, y_prize) = Self::line_to_values(&prize_info, "=");
        ClawMachine {
            x_a,
            y_a,
            x_b,
            y_b,
            x_prize,
            y_prize,
        }
    }

    pub fn from_string_p2(input_str: &str) -> ClawMachine {
        let mut cm = Self::from_string(input_str);
        cm.x_prize += 10000000000000;
        cm.y_prize += 10000000000000;
        cm
    }

    pub fn get_solution(&self) -> Option<i64> {
        let det_a = self.x_prize * self.y_b - self.x_b * self.y_prize;
        let det_b = self.x_a * self.y_prize - self.x_prize * self.y_a;
        let det = self.x_a * self.y_b - self.x_b * self.y_a;
        if det == 0 {
            None
        } else {
            let (a, a_rem) = (det_a / det, det_a % det);
            let (b, b_rem) = (det_b / det, det_b % det);
            if a_rem != 0 || b_rem != 0 {
                None
            } else {
                Some(a * 3 + b)
            }
        }
    }
}

fn part1(input_str: &String) -> i64 {
    input_str
        .split("\n\n")
        .map(ClawMachine::from_string)
        .map(|cm| cm.get_solution().unwrap_or(0))
        .sum()
}
fn part2(input_str: &String) -> i64 {
    input_str
        .split("\n\n")
        .map(ClawMachine::from_string_p2)
        .map(|cm| cm.get_solution().unwrap_or(0))
        .sum()
}

fn main() {
    let demo_filename = "demo";
    let demo_str = std::fs::read_to_string(demo_filename).expect("Unable to read file");
    assert_eq!(480, part1(&demo_str));

    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    assert_eq!(27157, part1(&input_str));
    assert_eq!(104015411578548, part2(&input_str));
}
