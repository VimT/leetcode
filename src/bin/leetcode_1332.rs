//! 删除回文子序列

pub fn remove_palindrome_sub(s: String) -> i32 {
    let s = s.as_bytes();
    if s.len() == 1 { return 1; }
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] == s[r] {
            l += 1;
            r -= 1;
        } else {
            return 2;
        }
    }
    1
}

fn main() {
    assert_eq!(remove_palindrome_sub(String::from("ababa")), 1);
    assert_eq!(remove_palindrome_sub(String::from("abb")), 2);
    assert_eq!(remove_palindrome_sub(String::from("baabb")), 2);
}
