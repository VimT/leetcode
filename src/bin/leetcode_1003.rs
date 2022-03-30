//! 检查替换后的词是否有效

pub fn is_valid(s: String) -> bool {
    let s = s.as_bytes();
    if s.len() % 3 != 0 { return false; }
    let mut stack = vec![];
    for &ch in s {
        if ch == b'a' {
            stack.push(ch);
        } else if ch == b'b' {
            if !stack.is_empty() && *stack.last().unwrap() == b'a' {
                stack.push(b'b');
            } else {
                return false;
            }
        } else {
            if !stack.is_empty() && *stack.last().unwrap() == b'b' {
                stack.pop();
                stack.pop();
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn main() {
    assert_eq!(is_valid(String::from("aabcbc")), true);
    assert_eq!(is_valid(String::from("abcabcababcc")), true);
    assert_eq!(is_valid(String::from("abccba")), false);
}
