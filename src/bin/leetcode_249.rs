//! 移位字符串分组

use std::collections::HashMap;
use leetcode::{svec, unorder};

pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut group = HashMap::new();
    for str in strings {
        let s = str.as_bytes();
        let sub = s[0] - b'a';
        let mut target = s.to_vec();
        for ch in &mut target {
            *ch = (*ch + 26 - sub - b'a') % 26 + b'a';
        }
        group.entry(target).or_insert(vec![]).push(str);
    }
    group.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    fn test(func: fn(strings: Vec<String>) -> Vec<Vec<String>>) {
        assert_eq!(unorder(func(svec!["abc","bcd","acef","xyz","az","ba","a","z"])), unorder(vec![svec!["acef"], svec!["a","z"], svec!["abc","bcd","xyz"], svec!["az","ba"]]));
        assert_eq!(unorder(func(svec!["a"])), unorder(vec![svec!["a"]]));
    }
    test(group_strings);
}
