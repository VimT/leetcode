//! 字符串的索引对

use leetcode::svec;

/// kmp找多个
pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let query = text.as_bytes();
    let n = query.len();
    for word in words {
        let pattern = word.as_bytes();
        let m = pattern.len();
        let mut next = vec![0; m];
        let mut j = 0;
        for i in 1..m {
            // why while: aabaaa, last a need while
            while j > 0 && pattern[i] != pattern[j] {
                j = next[j - 1];
            }
            if pattern[i] == pattern[j] { j += 1; }
            next[i] = j;
        }
        j = 0;
        for i in 0..n {
            while j > 0 && query[i] != pattern[j] {
                j = next[j - 1];
            }
            if query[i] == pattern[j] { j += 1; }
            if j == m {
                result.push(vec![(i + 1 - m) as i32, i as i32]);
                j = next[j - 1];
            }
        }
    }
    result.sort_unstable();
    result
}

fn main() {
    fn test(func: fn(text: String, words: Vec<String>) -> Vec<Vec<i32>>) {
        assert_eq!(func(String::from("ababa"), svec!["aba","ab"]), vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]);
        assert_eq!(func(String::from("thestoryofleetcodeandme"), svec!["story","fleet","leetcode"]), vec![vec![3, 7], vec![9, 13], vec![10, 17]]);
    }
    test(index_pairs);
}
