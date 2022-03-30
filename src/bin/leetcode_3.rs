//! 无重复字符的最长子串

use std::collections::HashMap;

/// 滑动窗口，保存上一个出现的字符的位置，遇到重复的就去掉 前进到重复位置+1
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 { return 0; }
    let chars: Vec<char> = s.chars().collect();
    let mut map: HashMap<char, usize> = HashMap::new();
    map.insert(chars[0], 1);
    let mut max = 1;
    let mut left: usize = 0;
    for right in 1..s.len() {
        let c = chars[right];
        let last = *map.get(&c).unwrap_or(&left);
        if last > left {
            left = last;
        }
        let tmp = right - left + 1;
        if tmp > max {
            max = tmp;
        }
        map.insert(c, right + 1);
    }

    max as i32
}

fn main() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
}
