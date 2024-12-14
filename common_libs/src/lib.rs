pub mod lineq;
pub mod map;
pub mod point;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use gcd::Gcd;

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
    #[test]
    fn point_add() {
        use point::Point;
        let p = Point::new(1, 1);
        assert_eq!(Point::new(2, 2), p + p);
        assert_eq!(Point::new(2, 2), p + &p);
        assert_eq!(Point::new(2, 2), &p + p);
        assert_eq!(Point::new(2, 2), &p + &p);
        assert_eq!(Point::new(0, 0), p - p);
        assert_eq!(Point::new(0, 0), p - &p);
        assert_eq!(Point::new(0, 0), &p - p);
        assert_eq!(Point::new(0, 0), &p - &p);
    }
    #[test]
    fn point_add_assign() {
        use point::Point;
        let p = Point::new(3, 5);
        let mut p1 = p;
        let p2 = Point::new(1, 1);
        p1 += p2;
        assert_eq!(Point::new(4, 6), p1);
        p1 += &p2;
        assert_eq!(Point::new(5, 7), p1);
        let p3 = &mut p1;
        *p3 += p2;
        assert_eq!(Point::new(6, 8), *p3);
    }
    #[test]
    fn point_mul() {
        use point::Point;
        let p = Point::new(3, 5);
        assert_eq!(Point::new(6, 10), p * 2);
        assert_eq!(Point::new(6, 10), 2 * p);
        assert_eq!(Point::new(6, 10), &p * 2);
        assert_eq!(Point::new(6, 10), 2 * &p);
    }
    #[test]
    fn point_mul_assign() {
        use point::Point;
        let mut p = Point::new(3, 5);
        p *= 2;
        assert_eq!(Point::new(6, 10), p);
        let p = &mut p;
        *p *= -2;
        assert_eq!(Point::new(-12, -20), *p);
    }
}
