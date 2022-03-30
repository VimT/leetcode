//! 匹配子序列的单词数

use leetcode::svec;

/// 指向下一个字母的指针
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![[len; 26]; len + 1];
    for i in (0..len).rev() {
        dp[i] = dp[i + 1].clone();
        dp[i][(s[i] - b'a') as usize] = i;
    }
    let mut result = 0;
    for word in words {
        let w = word.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while j < w.len() {
            i = dp[i][(w[j] - b'a') as usize];
            if i == len { break; }
            i += 1;
            j += 1;
        }
        if j == w.len() { result += 1; }
    }
    result
}

/// 后缀匹配
pub fn num_matching_subseq_suffix(s: String, words: Vec<String>) -> i32 {
    let words = words.into_iter().map(|v| v.into_bytes()).collect::<Vec<_>>();
    let mut heads = vec![vec![]; 26];
    for word in &words {
        heads[(word[0] - b'a') as usize].push(&word[1..]);
    }
    let mut res = 0;
    for ch in s.bytes() {
        let tails = std::mem::take(&mut heads[(ch - b'a') as usize]);
        for tail in tails {
            if tail.is_empty() {
                res += 1;
            } else {
                heads[(tail[0] - b'a') as usize].push(&tail[1..]);
            }
        }
    }
    return res;
}

fn main() {
    assert_eq!(num_matching_subseq_suffix(String::from("abcde"), svec!["a", "bb", "acd", "ace"]), 3);
    assert_eq!(num_matching_subseq_suffix(String::from("dsahjpjauf"), svec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]), 2);
}
