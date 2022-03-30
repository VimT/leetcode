//! 模糊坐标

use leetcode::{svec, unorder};

pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let len = s.len();
    fn to_points(s: &[u8]) -> Vec<String> {
        if s.len() == 1 { unsafe { return vec![String::from_utf8_unchecked(s.to_vec())]; } }
        let mut all_zero = true;
        for &ch in s {
            if ch != b'0' {
                all_zero = false;
                break;
            }
        }
        if all_zero { return vec![]; }
        let mut tail_zero = 0;
        for &ch in s.iter().rev() {
            if ch == b'0' {
                tail_zero += 1;
            } else { break; }
        }
        if s[0] == b'0' {
            return if tail_zero > 0 {
                vec![]
            } else {
                vec![format!("0.{}", String::from_utf8_lossy(&s[1..]))]
            };
        }
        let mut result = vec![unsafe { String::from_utf8_unchecked(s.to_vec()) }];
        if tail_zero > 0 {
            return result;
        }
        for i in 1..s.len() {
            result.push(format!("{}.{}", String::from_utf8_lossy(&s[..i]), String::from_utf8_lossy(&s[i..])));
        }
        result
    }
    let mut result = Vec::new();
    for i in 2..len - 1 {
        let left = &s[1..i];
        let right = &s[i..len - 1];
        for ll in to_points(left) {
            for rr in to_points(right) {
                result.push(format!("({}, {})", ll, rr));
            }
        }
    }
    result
}

fn main() {
    assert_eq!(unorder(ambiguous_coordinates(String::from("(123)"))), unorder(svec!["(1, 2.3)", "(1, 23)", "(1.2, 3)", "(12, 3)"]));
    assert_eq!(unorder(ambiguous_coordinates(String::from("(0123)"))), unorder(svec!["(0, 1.23)", "(0, 12.3)", "(0, 123)", "(0.1, 2.3)", "(0.1, 23)", "(0.12, 3)"]));
    assert_eq!(unorder(ambiguous_coordinates(String::from("(00011)"))), unorder(svec!["(0, 0.011)", "(0.001, 1)"]));
}
