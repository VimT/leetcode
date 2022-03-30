//! 分数加减运算

use std::mem::swap;
use std::ops::AddAssign;
use std::str::FromStr;

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b { swap(&mut a, &mut b); }
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    return a;
}

#[derive(Debug, Copy, Clone)]
struct Num {
    m: i32,
    d: i32,
}

impl Num {
    fn reduce(&mut self) {
        if self.d != 0 && self.m != 0 {
            let g = gcd(self.m.abs(), self.d);
            self.m /= g;
            self.d /= g;
        }
        if self.m == 0 { self.d = 1; }
    }

    fn format(&self) -> String {
        format!("{}/{}", self.m, self.d)
    }
}

impl AddAssign for Num {
    fn add_assign(&mut self, rhs: Self) {
        let d = self.d * rhs.d;
        let m = self.m * rhs.d + rhs.m * self.d;
        self.d = d;
        self.m = m;
        self.reduce();
    }
}

impl FromStr for Num {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let len = s.len();
        let mut i = 0;
        let neg = s[0] == b'-';
        if !s[i].is_ascii_digit() { i += 1; }
        let mut m = 0;
        while i < len && s[i] != b'/' {
            m = m * 10 + (s[i] - b'0') as i32;
            i += 1;
        }
        i += 1;
        if neg { m = -m; }
        let mut d = 0;
        while i < len {
            d = d * 10 + (s[i] - b'0') as i32;
            i += 1;
        }
        if d == 0 { d = 1; }
        Ok(Num { m, d })
    }
}

pub fn fraction_addition(expression: String) -> String {
    let s = expression.as_bytes();
    let len = s.len();
    let mut num = Num::from_str("0").unwrap();
    let mut start = 0;
    while start < len {
        let mut end = start + 1;
        while end < len && (s[end] != b'-' && s[end] != b'+') { end += 1; }
        let rhs = Num::from_str(&expression[start..end]).unwrap();
        num += rhs;
        start = end;
    }
    num.format()
}

fn main() {
    assert_eq!(fraction_addition(String::from("-1/4-4/5-1/4")), String::from("-13/10"));
    assert_eq!(fraction_addition(String::from("-1/2+1/2")), String::from("0/1"));
    assert_eq!(fraction_addition(String::from("-1/2+1/2+1/3")), String::from("1/3"));
    assert_eq!(fraction_addition(String::from("1/3-1/2")), String::from("-1/6"));
    assert_eq!(fraction_addition(String::from("5/3+1/3")), String::from("2/1"));
}
