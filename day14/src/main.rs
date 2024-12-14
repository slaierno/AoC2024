use common_libs::map::Point;
use itertools::Itertools;
use std::collections::HashMap;

struct Robot {
    pub p: Point,
    v: Point,
    wh: (isize, isize),
}

impl Robot {
    pub fn from_string_wh(input_str: &str, wh: (isize, isize)) -> Robot {
        let (p0, v) = input_str
            .split_whitespace()
            .map(|x| x.split_once('=').unwrap().1)
            .map(Point::from_string)
            .collect_tuple()
            .unwrap();
        Robot { p: p0, v, wh }
    }
    pub fn tick_n(&mut self, n: isize) {
        self.p = self.p + n * self.v;
        self.p.x = self.p.x.rem_euclid(self.wh.0);
        self.p.y = self.p.y.rem_euclid(self.wh.1);
    }

    fn is_in_middle(&self) -> bool {
        let (w, h) = self.wh;
        let (x, y) = (self.p.x, self.p.y);
        (w % 2 != 0 && x == w / 2) || (h % 2 != 0 && y == h / 2)
    }
    fn get_quadrant(&self) -> isize {
        let (w, h) = self.wh;
        let (x, y) = (self.p.x, self.p.y);
        (y < h / 2) as isize + 2 * (x < w / 2) as isize
    }
}

fn part1_wh(input_str: &String, wh: (isize, isize)) -> usize {
    let time = 100;
    let robots = input_str.lines().map(|s| Robot::from_string_wh(s, wh));
    let quadrant_count: HashMap<isize, usize> = robots
        .map(|mut r| {
            r.tick_n(time);
            r
        })
        .filter(|r| !r.is_in_middle())
        .map(|r| r.get_quadrant())
        .fold(HashMap::new(), |mut acc, q| {
            *acc.entry(q).or_default() += 1;
            acc
        });
    quadrant_count.values().fold(1, |score, n| score * n)
}
fn part1(input_str: &String) -> usize {
    part1_wh(input_str, (101, 103))
}

fn get_as_image(robots: &Vec<Robot>, (w, h): (isize, isize)) -> image::GrayImage {
    let mut buf = image::GrayImage::new(w as u32, h as u32);
    for r in robots {
        buf.put_pixel(r.p.x as u32, r.p.y as u32, image::Luma([255]));
    }
    buf
}
#[allow(dead_code)]
fn part2_full_simulation(input_str: &String) {
    let wh = (101, 103);
    let mut robots = input_str
        .lines()
        .map(|s| Robot::from_string_wh(s, wh))
        .collect_vec();

    for t in 0..=wh.0 * wh.1 {
        get_as_image(&robots, wh)
            .save(format!("frames/frame-{}.png", t))
            .unwrap();
        robots.iter_mut().for_each(|r| r.tick_n(1));
    }
}

fn part2_single_frame(input_str: &String, time: isize) {
    let wh = (101, 103);
    let robots = input_str
        .lines()
        .map(|s| Robot::from_string_wh(s, wh))
        .map(|mut r| {
            r.tick_n(time);
            r
        })
        .collect_vec();

    get_as_image(&robots, wh)
        .save(format!("frames/frame-{}.png", time))
        .unwrap();
}

#[test]
fn demo() {
    let input_str = "\
            p=0,4 v=3,-3\n\
            p=6,3 v=-1,-3\n\
            p=10,3 v=-1,2\n\
            p=2,0 v=2,-1\n\
            p=0,0 v=1,3\n\
            p=3,0 v=-2,-2\n\
            p=7,6 v=-1,-3\n\
            p=3,0 v=-1,-2\n\
            p=9,3 v=2,3\n\
            p=7,3 v=-1,2\n\
            p=2,4 v=2,-3\n\
            p=9,5 v=-3,-3"
        .to_string();

    assert_eq!(12, part1_wh(&input_str, (11, 7)));
}

fn main() {
    let input_filename = "input";
    let input_str = std::fs::read_to_string(input_filename).expect("Unable to read file");
    assert_eq!(231782040, part1(&input_str));
    part2_single_frame(&input_str, 6475);
}
