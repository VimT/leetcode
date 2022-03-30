//! 括号的分数

pub fn score_of_parentheses(s: String) -> i32 {
    let mut stack = vec![];
    let s = s.as_bytes();
    let mut result = 0;
    let len = s.len();
    let mut n = 0;
    for i in 0..len {
        if s[i] == b'(' {
            stack.push(i);
            if n > 0 { n <<= 1; } else { n = 1; }
        } else {
            stack.pop();
            if s[i - 1] == b'(' {
                result += n;
            }
            n >>= 1;
        }
    }
    result
}

fn main() {
    assert_eq!(score_of_parentheses(String::from("()")), 1);
    assert_eq!(score_of_parentheses(String::from("(())")), 2);
    assert_eq!(score_of_parentheses(String::from("()()")), 2);
}
