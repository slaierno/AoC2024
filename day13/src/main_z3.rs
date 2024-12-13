use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, Optimize, SatResult,
};

macro_rules! enable_z3_macro {
    ($ctx:expr) => {
        macro_rules! z3i {
            ($n:expr) => {
                Int::from_i64($ctx, $n)
            };
        }
        #[allow(unused_macros)] // false positive
        macro_rules! z3if {
            ($n:expr) => {
                || Int::from_i64($ctx, $n)
            };
        }
        macro_rules! z3cf {
            ($n:expr) => {
                || Int::new_const($ctx, $n)
            };
        }
    };
}
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
            .split_once(":")
            .unwrap()
            .1
            .split_terminator(",")
            .map(|s| s.split_once(delimiter).unwrap().1.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap()
    }
    pub fn from_string(input_str: &str) -> ClawMachine {
        let mut lines = input_str.lines();
        let a_button = lines.next().unwrap();
        let b_button = lines.next().unwrap();
        let prize_info = lines.next().unwrap();
        assert_eq!(lines.next(), None);
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

    pub fn get_all_constraints<'ctx>(
        &self,
        ctx: &'ctx Context,
    ) -> impl Iterator<Item = z3::ast::Bool<'ctx>> {
        enable_z3_macro!(&ctx);
        let x_a = z3if!(self.x_a);
        let x_b = z3if!(self.x_b);
        let x_prize = z3if!(self.x_prize);
        let y_a = z3if!(self.y_a);
        let y_b = z3if!(self.y_b);
        let y_prize = z3if!(self.y_prize);
        let a = z3cf!("A");
        let b = z3cf!("B");
        [
            (a() * x_a() + b() * x_b())._eq(&x_prize()),
            (a() * y_a() + b() * y_b())._eq(&y_prize()),
            a().gt(&z3i!(0)),
            b().gt(&z3i!(0)),
        ]
        .into_iter()
    }
}

fn solve_claw_machine(cm: &ClawMachine) -> Option<i64> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);
    enable_z3_macro!(&ctx);

    let a = z3cf!("A");
    let b = z3cf!("B");
    let min_function = a() * z3i!(3) + b();
    cm.get_all_constraints(&ctx).for_each(|x| opt.assert(&x));
    opt.minimize(&min_function);

    if let SatResult::Sat = opt.check(&[]) {
        let model = opt.get_model().unwrap();
        let a_res: i64 = model.eval(&a(), true).unwrap().as_i64().unwrap();
        let b_res = model.eval(&b(), true).unwrap().as_i64().unwrap();
        Some(a_res * 3 + b_res)
    } else {
        None
    }
}

fn part1(input_str: &String) -> i64 {
    input_str
        .split("\n\n")
        .map(ClawMachine::from_string)
        .map(|cm| solve_claw_machine(&cm).unwrap_or(0))
        .sum()
}
fn part2(input_str: &String) -> i64 {
    input_str
        .split("\n\n")
        .map(ClawMachine::from_string_p2)
        .map(|cm| solve_claw_machine(&cm).unwrap_or(0))
        .sum()
}

fn test() {
    {
        let instructions_str = "Button A: X+94, Y+34\n\
                                  Button B: X+22, Y+67\n\
                                  Prize: X=8400, Y=5400"
            .to_string();

        let cm = ClawMachine::from_string(&instructions_str.to_string());
        assert_eq!(solve_claw_machine(&cm), Some(280));
    }
    {
        let demo_str = "Button A: X+94, Y+34\n\
                              Button B: X+22, Y+67\n\
                              Prize: X=8400, Y=5400\n\
                              \n\
                              Button A: X+26, Y+66\n\
                              Button B: X+67, Y+21\n\
                              Prize: X=12748, Y=12176\n\
                              \n\
                              Button A: X+17, Y+86\n\
                              Button B: X+84, Y+37\n\
                              Prize: X=7870, Y=6450\n\
                              \n\
                              Button A: X+69, Y+23\n\
                              Button B: X+27, Y+71\n\
                              Prize: X=18641, Y=10279"
            .to_string();
        assert_eq!(480, part1(&demo_str));
        // They didn't even bother us to give a solution for part 2 example!
    }
}

fn main() {
    test();

    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    assert_eq!(27157, part1(&input_str));
    assert_eq!(104015411578548, part2(&input_str));
}
