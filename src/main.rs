use std::ops::{Add, Sub};
use std::fmt;

fn main() {
    println!("Hello, world!");
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug)]
enum CalcError {
    Zero,
    Parse,
}

struct Rational {
    n: i64,
    d: i64,
}

impl Rational {
    fn new(n: i64, d: i64) -> Result<Rational, CalcError> {
        if d == 0 {
            Err(CalcError::Zero)
        } else {
            Ok(Rational { n: n, d: d })
        }
    }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Rational {
        Rational::new(n, 1).unwrap()
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.d == 1 {
            write!(f, "{}", self.n)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        Rational {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
