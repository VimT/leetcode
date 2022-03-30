//! 字母异位词分组


use std::collections::HashMap;

use leetcode::{svec, unorder_deep};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    fn counter(s: &str) -> [u8; 26] {
        let mut ans = [0; 26];
        for i in s.bytes() {
            ans[(i - b'a') as usize] += 1;
        }
        ans
    }
    for i in strs {
        map.entry(counter(i.as_str())).or_insert(Vec::new()).push(i);
    }
    map.into_iter().map(|x| x.1).collect()
}

/// 一个很巧妙的算法是将a-z 用质数来表示，不同质数的乘积必定不同，利用此原理进行分组,很巧妙，很经典
pub fn group_anagrams_prime(strs: Vec<String>) -> Vec<Vec<String>> {
    let prime = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
        41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
        89, 97, 101,
    ];
    let mut m = HashMap::new();
    let mut v = vec![];
    for s in strs {
        let mut times = 1;
        for b in s.as_bytes() {
            times *= prime[(b - b'a') as usize];
        }
        (*m.entry(times).or_insert(vec![])).push(s);
    }
    for (_, value) in m {
        v.push(value);
    }
    v
}


fn main() {
    fn test(func: fn(strs: Vec<String>) -> Vec<Vec<String>>) {
        assert_eq!(unorder_deep(func(svec!["eat", "tea", "tan", "ate", "nat", "bat"])), unorder_deep(vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]));
        assert_eq!(unorder_deep(func(svec![""])), unorder_deep(vec![vec![""]]));
        assert_eq!(unorder_deep(func(svec!["a"])), unorder_deep(vec![vec!["a"]]));
    }
    test(group_anagrams);
    test(group_anagrams_prime);
}
