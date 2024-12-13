pub mod lineq;
pub mod map;
pub mod point;
use gcd::Gcd;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn check_bezout(a: i64, b: i64, c: i64) {
        let bezout = lineq::Bezout::new(a, b, c);
        if let Some(((x, y), g)) = bezout.find_one_solution() {
            assert_eq!(x * a + y * b, c);
            assert_eq!((a.abs() as u64).gcd(b.abs() as u64), g.abs() as u64);
        } else {
            unreachable!()
        };
    }
    #[test]
    fn bezout() {
        check_bezout(14, 5, 73);
        check_bezout(5, 14, 73);
        check_bezout(258, 147, 369);
        check_bezout(147, 258, 369);
    }
}
