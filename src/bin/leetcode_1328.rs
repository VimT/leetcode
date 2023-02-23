//! 破坏回文串

pub fn break_palindrome(mut palindrome: String) -> String {
    if palindrome.len() == 1 { return String::new(); }
    let s = unsafe { palindrome.as_bytes_mut() };
    let len = s.len();
    let mut change = false;
    for i in 0..len / 2 {
        if s[i] != b'a' {
            s[i] = b'a';
            change = true;
            break;
        }
    }
    if !change {
        s[len - 1] += 1;
    }
    palindrome
}

fn main() {
    fn test(func: fn(palindrome: String) -> String) {
        assert_eq!(func(String::from("abccba")), String::from("aaccba"));
        assert_eq!(func(String::from("a")), String::from(""));
    }
    test(break_palindrome);
}
