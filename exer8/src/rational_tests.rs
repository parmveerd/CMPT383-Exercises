#[cfg(test)]
use crate::rational::Rational;

#[test]
fn rational_test() {
    let mut r = Rational::new(6, 8);
    assert_eq!(format!("{:?}", r), "Rational { n: 6, d: 8 }");
    r.reduce();
    assert_eq!(format!("{:?}", r), "Rational { n: 3, d: 4 }");
    let four1 = Rational::from(4);
    let four2 = Rational::new(4, 1);
    assert_eq!(four1, four2);
}

#[test]
fn rational_new() {
    let r = Rational::new(3, 5);
    assert_eq!(format!("{}", r), "3/5");
}

#[test]
fn rational_reduce() {
    let mut r = Rational::new(12, 20);
    r.reduce();
    assert_eq!(format!("{}", r), "3/5");
}

#[test]
fn from_i64() {
    let r: Rational = 4_i64.into();
    assert_eq!(format!("{}", r), "4/1");
}

#[test]
fn into_f64() {
    let r = Rational::new(3, 4);
    let f: f64 = r.into();
    assert_eq!(f, 0.75);
}

#[test]
fn rational_display() {
    let r = Rational::new(-6, 8);
    assert_eq!(format!("{}", r), "-6/8");
}

