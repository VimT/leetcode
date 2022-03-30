//! 特殊等价字符串组

use std::collections::HashSet;

use leetcode::svec;

pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
    let mut set = HashSet::new();
    for s in words {
        let s = s.as_bytes();
        let mut count = [0; 52];
        for i in 0..s.len() {
            count[(s[i] - b'a') as usize + 26 * (1 - i % 2)] += 1;
        }
        set.insert(count);
    }
    set.len() as i32
}

fn main() {
    assert_eq!(num_special_equiv_groups(svec!["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]), 3);
    assert_eq!(num_special_equiv_groups(svec!["abc", "acb", "bac", "bca", "cab", "cba"]), 3);
}