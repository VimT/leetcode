//! 子串的最大出现次数

use std::collections::HashMap;

pub fn max_freq(s: String, max_letters: i32, min_size: i32, _: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut ch_cnt = [0; 26];
    let mut diff_cnt = 0;
    let mut i = 0;
    let min_size = min_size as usize;
    let mut map: HashMap<&[u8], i32> = HashMap::new();
    for j in 0..len {
        let idx = (s[j] - b'a') as usize;
        ch_cnt[idx] += 1;
        if ch_cnt[idx] == 1 {
            diff_cnt += 1;
        }
        while diff_cnt > max_letters || (j - i + 1) > min_size {
            ch_cnt[(s[i] - b'a') as usize] -= 1;
            if ch_cnt[(s[i] - b'a') as usize] == 0 {
                diff_cnt -= 1;
            }
            i += 1;
        }
        if j - i + 1 == min_size {
            *map.entry(&s[i..=j]).or_default() += 1;
        }
    }
    map.values().max().cloned().unwrap_or(0)
}

fn main() {
    fn test(func: fn(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32) {
        assert_eq!(func(String::from("aababcaab"), 2, 3, 4), 2);
        assert_eq!(func(String::from("aaaa"), 1, 3, 3), 2);
        assert_eq!(func(String::from("abcde"), 2, 3, 3), 0);
    }
    test(max_freq);
}
