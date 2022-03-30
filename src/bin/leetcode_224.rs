//! 基本计算器


use std::collections::VecDeque;

pub fn calculate(s: String) -> i32 {
    let s = s.as_bytes();
    let mut stack = VecDeque::new();
    let mut operand = 0;
    let mut ans = 0;
    let mut sign = 1;
    for &i in s {
        match i {
            b'0'..=b'9' => {
                operand = operand * 10 + (i - b'0') as i32;
            }
            b'+' => {
                ans += sign * operand;
                sign = 1;
                operand = 0;
            }
            b'-' => {
                ans += sign * operand;
                sign = -1;
                operand = 0;
            }
            b'(' => {
                stack.push_back(ans);
                stack.push_back(sign);
                sign = 1;
                ans = 0;
            }
            b')' => {
                ans += sign * operand;
                ans *= stack.pop_back().unwrap();
                ans += stack.pop_back().unwrap();
                operand = 0;
            }
            _ => {}
        }
    }
    ans + sign * operand
}

fn main() {
    assert_eq!(calculate(String::from("1 + 1")), 2);
    assert_eq!(calculate(String::from(" 2-1 + 2 ")), 3);
    assert_eq!(calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
}
