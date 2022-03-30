//! 有效的括号

pub fn is_valid(s: String) -> bool {
    let mut stack = std::collections::VecDeque::new();
    for i in s.bytes() {
        if i == b'(' || i == b'{' || i == b'[' {
            stack.push_back(i);
        } else {
            if stack.is_empty() { return false; }
            match stack.pop_back().unwrap() {
                b'(' => if i != b')' { return false; }
                b'{' => if i != b'}' { return false; }
                b'[' => if i != b']' { return false; }
                _ => return false
            }
        }
    }
    return stack.is_empty();
}

fn main() {
    assert_eq!(is_valid(String::from("()")), true);
    assert_eq!(is_valid(String::from("()[]{}")), true);
    assert_eq!(is_valid(String::from("(]")), false);
}
