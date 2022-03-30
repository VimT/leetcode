//! 重复的DNA序列

use std::collections::{HashMap, HashSet};

use leetcode::unorder;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let mut set = HashSet::new();
    let mut ans = HashSet::new();
    for win in s.windows(10) {
        if set.contains(win) {
            unsafe { ans.insert(String::from_utf8_unchecked(win.to_vec())); }
        } else {
            set.insert(win);
        }
    }
    ans.into_iter().collect()
}

pub fn find_repeated_dna_sequences_rabin_karp(s: String) -> Vec<String> {
    let sb = s.as_bytes();
    let s: Vec<u64> = s.as_bytes().iter().map(|x| match x {
        b'A' => 0,
        b'C' => 1,
        b'G' => 2,
        b'T' => 3,
        _ => panic!("unknown dna")
    }).collect();
    let len = s.len();
    if len <= 10 { return vec![]; }
    let mut ans = vec![];
    let a = 4_u64;
    let al = a.pow(10);
    let mut seen = HashMap::new();
    let mut h = 0;
    for i in 0..10 {
        h = h * a + s[i];
    }
    seen.insert(h, 1);
    unsafe {
        for start in 1..len - 9 {
            h = h * a - s[start - 1] * al + s[start + 9];
            *seen.entry(h).or_insert(0) += 1;
            if *seen.entry(h).or_insert(0) == 2 {
                ans.push(String::from_utf8_unchecked(sb[start..start + 10].to_vec()));
            }
        }
    }
    ans
}

fn main() {
    fn test(func: fn(s: String) -> Vec<String>) {
        assert_eq!(unorder(func(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"))), ["AAAAACCCCC", "CCCCCAAAAA"]);
        assert_eq!(unorder(func(String::from("AAAAAAAAAAAAA"))), ["AAAAAAAAAA"]);
    }
    test(find_repeated_dna_sequences);
    test(find_repeated_dna_sequences_rabin_karp);
}
