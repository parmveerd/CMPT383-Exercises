use std::fmt;

fn gcd(a: i64, b: i64) -> i64 {
    // TODO
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
    // TODO
    n: i64,
    d: i64,
}

impl Rational {
    pub fn new(n: i64, d: i64) -> Rational {
        // TODO
        let gcd = gcd(n, d);
        Self { n: n, d: d}
    }
    // TODO: the reduce method
    pub fn reduce(&mut self) {
        let gcd = gcd(self.n, self.d);
        self.n /= gcd;
        self.d /= gcd;
    }
}

impl From<i64> for Rational {
    // TODO
    fn from(num: i64) -> Self {
        Self { n: num, d: 1 }
    }
}


impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

impl From<Rational> for f64 {
    fn from(rational: Rational) -> f64 {
        rational.n as f64 / rational.d as f64
    }
}

