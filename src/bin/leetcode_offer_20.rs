//! 剑指 Offer 20. 表示数值的字符串

pub fn is_number(s: String) -> bool {
    fn at_least_one_num(s: &[u8], i: &mut usize) -> bool {
        if *i >= s.len() {
            return false;
        }
        if !(s[*i] >= b'0' && s[*i] <= b'9') {
            return false;
        }
        while *i < s.len() && s[*i] >= b'0' && s[*i] <= b'9' {
            *i += 1;
        }
        true
    }
    fn is_decimal(s: &[u8], i: &mut usize) -> bool {
        if *i >= s.len() {
            return false;
        }
        if s[*i] == b'+' || s[*i] == b'-' {
            *i += 1;
        }
        if *i >= s.len() {
            return false;
        }
        let next = s[*i];
        if next == b'.' {
            *i += 1;
            return at_least_one_num(s, i);
        }
        if !at_least_one_num(s, i) {
            return false;
        }
        if *i >= s.len() {
            return false;
        }
        if s[*i] != b'.' {
            return false;
        }
        *i += 1;
        while *i < s.len() && s[*i] >= b'0' && s[*i] <= b'9' {
            *i += 1;
        }
        true
    }
    fn is_zhengshu(s: &[u8], i: &mut usize) -> bool {
        if *i >= s.len() { return false; }
        if s[*i] == b'+' || s[*i] == b'-' {
            *i += 1;
        }
        at_least_one_num(s, i)
    }
    let s = s.as_bytes();
    let mut i = 0;
    while i < s.len() && s[i] == b' ' {
        i += 1;
    }
    let cur = i;
    if !is_decimal(s, &mut i) {
        i = cur;
        if !is_zhengshu(s, &mut i) {
            return false;
        }
    }

    if i < s.len() && (s[i] == b'e' || s[i] == b'E') {
        i += 1;
        if !is_zhengshu(s, &mut i) {
            return false;
        }
    }
    while i < s.len() && s[i] == b' ' {
        i += 1;
    }

    i == s.len()
}

#[derive(Eq, PartialEq)]
enum State {
    Init,
    IntSign,
    Int,
    Point,
    PointWithoutInt,
    Fraction,
    Exp,
    ExpSign,
    ExpNumber,
    End,
    Bad,
}

impl State {
    fn is_ok(&self) -> bool {
        match self {
            State::Int | State::Point | State::Fraction | State::ExpNumber | State::End => true,
            _ => false
        }
    }
}

enum CharType {
    Number,
    Exp,
    Point,
    Sign,
    Space,
    Illegal,
}

impl From<u8> for CharType {
    fn from(c: u8) -> Self {
        match c {
            b'0'..=b'9' => CharType::Number,
            b'e' | b'E' => CharType::Exp,
            b'.' => CharType::Point,
            b'+' | b'-' => CharType::Sign,
            b' ' => CharType::Space,
            _ => CharType::Illegal,
        }
    }
}

/// 有限状态自动机
fn is_number_a(s: String) -> bool {
    let s = s.as_bytes();
    let mut state = State::Init;
    for i in s {
        let ct = CharType::from(*i);
        state = match state {
            State::Init => {
                match ct {
                    CharType::Number => State::Int,
                    CharType::Point => State::PointWithoutInt,
                    CharType::Sign => State::IntSign,
                    CharType::Space => State::Init,
                    _ => State::Bad,
                }
            }
            State::IntSign => {
                match ct {
                    CharType::Number => State::Int,
                    CharType::Point => State::PointWithoutInt,
                    _ => State::Bad,
                }
            }
            State::Int => {
                match ct {
                    CharType::Number => State::Int,
                    CharType::Exp => State::Exp,
                    CharType::Point => State::Point,
                    CharType::Space => State::End,
                    _ => State::Bad,
                }
            }
            State::Point => {
                match ct {
                    CharType::Number => State::Fraction,
                    CharType::Exp => State::Exp,
                    CharType::Space => State::End,
                    _ => State::Bad,
                }
            }
            State::PointWithoutInt => {
                match ct {
                    CharType::Number => State::Fraction,
                    _ => State::Bad,
                }
            }
            State::Fraction => {
                match ct {
                    CharType::Number => State::Fraction,
                    CharType::Exp => State::Exp,
                    CharType::Space => State::End,
                    _ => State::Bad,
                }
            }
            State::Exp => {
                match ct {
                    CharType::Number => State::ExpNumber,
                    CharType::Sign => State::ExpSign,
                    _ => State::Bad,
                }
            }
            State::ExpSign => {
                match ct {
                    CharType::Number => State::ExpNumber,
                    _ => State::Bad,
                }
            }
            State::ExpNumber => {
                match ct {
                    CharType::Number => State::ExpNumber,
                    CharType::Space => State::End,
                    _ => State::Bad,
                }
            }
            State::End => {
                match ct {
                    CharType::Space => State::End,
                    _ => State::Bad,
                }
            }
            State::Bad => State::Bad
        };
        if state == State::Bad {
            return false;
        }
    }
    state.is_ok()
}

fn main() {
    assert!(!is_number_a(String::from("")));
    assert!(is_number_a(String::from("0")));
    assert!(!is_number_a(String::from("e")));
    assert!(!is_number_a(String::from(".")));
    assert!(is_number_a(String::from("        .1      ")));
    for i in ["+100", "5e2", "-123", "3.1416", "-1E-16", "0123"] {
        assert!(is_number_a(String::from(i)));
    }
    for i in ["12e", "1a3.14", "1.2.3", "+-5", "12e+5.4"] {
        assert!(!is_number_a(String::from(i)));
    }
}