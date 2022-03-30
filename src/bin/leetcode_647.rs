//! 回文子串

pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    #[inline]
    fn expend(s: &[u8], mut left: i32, mut right: i32) -> i32 {
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        right - 1 - left
    }
    let mut result = 0;
    for i in 0..len {
        let l1 = expend(s, i as i32, i as i32);
        let l2 = expend(s, i as i32, (i + 1) as i32);
        result += (l1 + 1) / 2;
        result += l2 / 2;
    }
    result
}

fn main() {
    assert_eq!(count_substrings(String::from("abc")), 3);
    assert_eq!(count_substrings(String::from("aaa")), 6);
}
