//! 判断一个括号字符串是否有效

pub fn can_be_valid(s: String, locked: String) -> bool {
    let s = s.as_bytes();
    let l = locked.as_bytes();
    let len = s.len();
    if len & 1 == 1 { return false; }
    let mut left = 0;
    let mut right = 0;
    for i in 0..len {
        if s[i] == b')' && l[i] == b'1' {
            right += 1;
            if i + 1 - right < right {
                return false;
            }
        }
    }
    for i in (0..len).rev() {
        if s[i] == b'(' && l[i] == b'1' {
            left += 1;
            if len - i - left < left {
                return false;
            }
        }
    }
    true
}

fn main() {
    assert_eq!(can_be_valid(String::from("((()(()()))()((()()))))()((()(()"), String::from("10111100100101001110100010001001")), true);
    assert_eq!(can_be_valid(String::from("((()"), String::from("1101")), true);
    assert_eq!(can_be_valid(String::from("))()))"), String::from("010100")), true);
    assert_eq!(can_be_valid(String::from("()()"), String::from("0000")), true);
    assert_eq!(can_be_valid(String::from(")"), String::from("0")), false);
}
