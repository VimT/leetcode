//! 单词子集

use leetcode::svec;

pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut cnt2 = [0; 26];
    for word in words2 {
        let mut cnt = [0; 26];
        for &ch in word.as_bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }
        for i in 0..26 {
            cnt2[i] = cnt2[i].max(cnt[i]);
        }
    }
    words1.into_iter().filter(|x| {
        let mut cnt = [0; 26];
        for &ch in x.as_bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if cnt[i] < cnt2[i] { return false; }
        }
        true
    }).collect()
}

fn main() {
    assert_eq!(word_subsets(svec!["amazon", "apple", "facebook", "google", "leetcode"], svec!["e", "o"]), svec!["facebook", "google", "leetcode"]);
    assert_eq!(word_subsets(svec!["amazon", "apple", "facebook", "google", "leetcode"], svec!["l", "e"]), svec!["apple", "google", "leetcode"]);
}
