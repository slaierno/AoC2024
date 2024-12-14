pub struct Bezout {
    a: i64,
    b: i64,
    c: i64,
}

impl Bezout {
    pub fn new(a: i64, b: i64, c: i64) -> Bezout {
        Bezout { a: a, b: b, c: c }
    }

    fn gcd(a: i64, b: i64, (x, y): (i64, i64)) -> (i64, (i64, i64)) {
        if b == 0 {
            (a, (1, 0))
        } else {
            let (div, rem) = (a / b, a % b);
            let (g, (x1, y1)) = Bezout::gcd(b, rem, (x, y));
            (g, (y1, x1 - div * y1))
        }
    }
    pub fn find_one_solution(&self) -> Option<((i64, i64), i64)> {
        if self.a == self.b {
            Some(((0, self.c), 1))
        } else if self.a == 0 || self.b == 0 {
            None // this is actually a one-variable equation
        } else {
            let (g, (x, y)) = Bezout::gcd(self.a, self.b, (0, 1));
            if self.c % g != 0 {
                None
            } else {
                let x0 = self.a.signum() * x * self.c / g;
                let y0 = self.b.signum() * y * self.c / g;
                Some(((x0, y0), g))
            }
        }
    }
}
