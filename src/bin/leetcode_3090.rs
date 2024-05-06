//! 每个字符最多出现两次的最长子字符串

pub fn maximum_length_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut result = 0;
    let mut left = 0;
    let mut cnt = [0; 26];
    for (right, &ch) in s.iter().enumerate() {
        let idx = (ch - b'a') as usize;
        cnt[idx] += 1;
        if cnt[idx] > 2 {
            while cnt[idx] > 2 {
                cnt[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
        }
        result = result.max(right - left + 1);
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("bcbbbcba")), 4);
        assert_eq!(func(String::from("aaaa")), 2);
    }
    test(maximum_length_substring);
}
