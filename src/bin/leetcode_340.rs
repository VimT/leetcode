//! 至多包含 K 个不同字符的最长子串

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut cnt = HashMap::new();
    let len = s.len();
    let k = k as usize;
    let mut result = 0;
    let mut i = 0;
    for j in 0..len {
        *cnt.entry(s[j]).or_insert(0) += 1;
        while cnt.len() > k {
            if let Entry::Occupied(mut v) = cnt.entry(s[i]) {
                *v.get_mut() -= 1;
                if *v.get() == 0 {
                    v.remove();
                }
            }
            i += 1;
        }
        result = result.max(j + 1 - i);
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("a"), 2), 1);
        assert_eq!(func(String::from("eceba"), 2), 3);
        assert_eq!(func(String::from("aa"), 1), 2);
    }
    test(length_of_longest_substring_k_distinct);
}
