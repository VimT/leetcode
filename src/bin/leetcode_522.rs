//! 最长特殊序列 II

use std::collections::HashMap;

use leetcode::svec;

pub fn find_lu_slength(strs: Vec<String>) -> i32 {
    let len = strs.len();
    let mut set: HashMap<Vec<u8>, u32> = HashMap::new();
    for i in 0..len {
        let s = strs[i].as_bytes();
        for j in 1i32..1 << s.len() {
            let mut vec = Vec::with_capacity(j.count_ones() as usize);
            for k in 0..s.len() {
                if j >> k & 1 == 1 {
                    vec.push(s[k]);
                }
            }
            let val = set.entry(vec).or_default();
            *val |= 1 << i;
        }
    }
    let mut result = -1;
    for (str, cnt) in set {
        if cnt.count_ones() == 1 {
            result = result.max(str.len() as i32);
        }
    }
    result
}

/// 如果存在这样的特殊序列，那么它一定是某个给定的字符串。
pub fn find_lu_slength_check(mut strs: Vec<String>) -> i32 {
    strs.sort_unstable_by_key(|x| x.len());
    let len = strs.len();
    let mut result = -1;
    #[inline]
    fn is_sub_seq(s1: &[u8], s2: &[u8]) -> bool {
        let mut i = 0;
        for j in 0..s2.len() {
            if s1[i] == s2[j] {
                i += 1;
                if i == s1.len() { break; }
            }
        }
        i == s1.len()
    }
    for i in 0..len {
        let mut ok = true;
        for j in 0..len {
            if i == j { continue; }
            if is_sub_seq(strs[i].as_bytes(), strs[j].as_bytes()) {
                ok = false;
                break;
            }
        }
        if ok {
            result = result.max(strs[i].len() as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(find_lu_slength_check(svec!["aba", "cdc", "eae"]), 3);
    assert_eq!(find_lu_slength(svec!["aba", "cdc", "eae"]), 3);
    assert_eq!(find_lu_slength_check(svec!["aaa", "aaa", "aa"]), -1);
    assert_eq!(find_lu_slength(svec!["aaa", "aaa", "aa"]), -1);
}
