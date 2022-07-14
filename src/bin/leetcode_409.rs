//! 最长回文串

pub fn longest_palindrome(s: String) -> i32 {
    let s = s.as_bytes();
    let mut sv = [0; 128];
    for &ch in s {
        sv[ch as usize] += 1;
    }
    let mut has_single = false;
    let mut result = 0;
    for mut num in sv {
        if num & 1 == 1 {
            has_single = true;
            num -= 1;
        }
        result += num;
    }
    result + has_single as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abccccdd")), 7);
        assert_eq!(func(String::from("a")), 1);
        assert_eq!(func(String::from("bb")), 2);
    }
    test(longest_palindrome);
}
