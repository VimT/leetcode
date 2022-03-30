//! 有效的括号字符串

pub fn check_valid_string(s: String) -> bool {
    let mut star = vec![];
    let mut left = vec![];
    let s = s.as_bytes();
    for i in 0..s.len() {
        match s[i] {
            b'(' => left.push(i),
            b')' => {
                if !left.is_empty() {
                    left.pop().unwrap();
                } else if !star.is_empty() {
                    star.pop().unwrap();
                } else {
                    return false;
                }
            }
            b'*' => star.push(i),
            _ => panic!()
        }
    }
    while !left.is_empty() && !star.is_empty() {
        let left_idx = left.pop().unwrap();
        let start_idx = star.pop().unwrap();
        if left_idx > start_idx {
            return false;
        }
    }
    left.is_empty()
}

fn main() {
    assert_eq!(check_valid_string("()".into()), true);
    assert_eq!(check_valid_string("(*)".into()), true);
    assert_eq!(check_valid_string("(*))".into()), true);
    assert_eq!(check_valid_string("".into()), true);
    assert_eq!(check_valid_string("*".into()), true);
    assert_eq!(check_valid_string(")".into()), false);
    assert_eq!(check_valid_string("((())".into()), false);
    assert_eq!(check_valid_string("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".into()), false);
}
