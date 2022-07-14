//! 至多包含两个不同字符的最长子串

use std::collections::HashMap;

/// 双指针，会有重复遍历，但是实测0ms
pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut left = 0;
    let mut result = 0;
    while left < len {
        let ch1 = s[left];
        let mut right = left + 1;
        while right < len && s[right] == ch1 {
            right += 1;
        }
        if right == len {
            result = result.max(right - left);
            break;
        }
        let ch2 = s[right];
        let mid = right;
        while right < len && (s[right] == ch1 || s[right] == ch2) {
            right += 1;
        }
        result = result.max(right - left);
        left = mid;
    }
    result as i32
}

/// map存每个字符出现的最后位置，方便left指针快速跳跃，理论应该更快，但实测32ms
pub fn length_of_longest_substring_two_distinct_map(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut left = 0;
    let mut last_pos = HashMap::new();
    let mut result = 0;
    let mut right = 0;
    while right < len {
        if last_pos.len() < 3 {
            last_pos.insert(s[right], right);
            right += 1;
        }
        if last_pos.len() == 3 {
            let del_idx = *last_pos.values().min().unwrap();
            last_pos.remove(&s[del_idx]);
            left = del_idx + 1;
        }
        result = result.max(right - left);
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("eceba")), 3);
        assert_eq!(func(String::from("ccaabbb")), 5);
    }
    test(length_of_longest_substring_two_distinct);
    test(length_of_longest_substring_two_distinct_map);
}
