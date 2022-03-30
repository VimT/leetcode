//! 相等的有理数

use std::str::FromStr;

struct Num {
    integer: i32,
    non_repeating: Vec<u8>,
    repeating: Vec<u8>,
}

impl FromStr for Num {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((int, other)) = s.split_once('.') {
            let int = int.parse().unwrap();
            let non;
            let mut rep = vec![];
            if let Some((none, rep_str)) = other.split_once('(') {
                non = none.as_bytes().iter().map(|x| *x - b'0').collect();
                rep = rep_str[..rep_str.len() - 1].as_bytes().iter().map(|x| *x - b'0').collect();
            } else {
                non = other.as_bytes().iter().map(|x| *x - b'0').collect();
            }
            return Ok(Num { integer: int, non_repeating: non, repeating: rep });
        }
        Ok(Num { integer: s.parse().unwrap(), non_repeating: vec![], repeating: vec![] })
    }
}

fn is_all9(arr: &[u8]) -> bool {
    for num in arr {
        if *num != 9 { return false; }
    }
    true
}

impl Num {
    fn decimal_about(&self) -> Vec<u8> {
        let mut result = self.non_repeating.clone();
        let mut rep = self.repeating.clone();
        if rep.is_empty() { rep.push(0); }
        while result.len() < 100 {
            result.extend_from_slice(&rep);
        }
        unsafe { result.set_len(100); }
        result
    }

    fn to_fixed(&self) -> (i32, Vec<u8>) {
        let mut int = self.integer;
        let mut decimal = self.decimal_about();
        if is_all9(&decimal) {
            int += 1;
            decimal.fill(0);
        } else if is_all9(&decimal[50..]) {
            for i in (0..50).rev() {
                if decimal[i] != 9 {
                    decimal[i] += 1;
                    decimal[i + 1..].fill(0);
                    break;
                }
            }
        }
        (int, decimal)
    }
}

pub fn is_rational_equal(s: String, t: String) -> bool {
    Num::from_str(&s).unwrap().to_fixed() == Num::from_str(&t).unwrap().to_fixed()
}

fn main() {
    assert_eq!(is_rational_equal(String::from("1.9(0)"), String::from("1.8(9)")), true);
    assert_eq!(is_rational_equal(String::from("0.(52)"), String::from("0.5(25)")), true);
    assert_eq!(is_rational_equal(String::from("0.1666(6)"), String::from("0.166(66)")), true);
    assert_eq!(is_rational_equal(String::from("0.9(9)"), String::from("1.")), true);
}
