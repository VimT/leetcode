//! 删除最外层的括号

pub fn remove_outer_parentheses(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = Vec::with_capacity(len);
    let mut stack = vec![];
    for i in 0..len {
        if s[i] == b'(' {
            stack.push(i);
        } else {
            let start = stack.pop().unwrap();
            if stack.is_empty() {
                result.extend_from_slice(&s[start + 1..i]);
            }
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(remove_outer_parentheses(String::from("(()())(())")), "()()()");
    assert_eq!(remove_outer_parentheses(String::from("(()())(())(()(()))")), "()()()()(())");
    assert_eq!(remove_outer_parentheses(String::from("()()")), "");
}
