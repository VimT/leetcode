//! 回文排列

pub fn can_permute_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut odd = 0;
    for i in 0..26 {
        if cnt[i] & 1 == 1 {
            odd += 1;
        }
    }
    odd <= 1
}

fn main() {
    fn test(func: fn(s: String) -> bool) {
        assert_eq!(func(String::from("aab")), true);
        assert_eq!(func(String::from("code")), false);
        assert_eq!(func(String::from("carerac")), true);
    }
    test(can_permute_palindrome);
}
