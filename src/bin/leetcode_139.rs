//! 单词拆分

use std::collections::VecDeque;

use leetcode::svec;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    fn inner(s: &String, word_dict: &Vec<String>, start: usize) -> bool {
        if s.len() == start {
            return true;
        }
        for i in word_dict {
            if s[start..].starts_with(i) {
                if inner(s, word_dict, start + i.len()) { return true; }
            };
        }
        false
    }
    return inner(&s, &word_dict, 0);
}

pub fn word_break_bfs(s: String, word_dict: Vec<String>) -> bool {
    let mut visited = vec![false; s.len()];
    let mut queue = VecDeque::new();
    queue.push_back(0);

    while !queue.is_empty() {
        let start = queue.pop_front().unwrap();
        if !visited[start] {
            for end in start + 1..=s.len() {
                let str = s[start..end].to_string();
                if word_dict.contains(&str) {
                    queue.push_back(end);
                    if end == s.len() {
                        return true;
                    }
                }
            }
        }
        visited[start] = true;
    }
    false
}

/// 动态规划，dp[i] 表示s[0..i] 是否是单词拆分
pub fn word_break_dp(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..dp.len() {
        for start in 0..i {
            if dp[start] && word_dict.contains(&s[start..i].to_string()) {
                dp[i] = true;
                break;
            }
        }
    }
    return *dp.last().unwrap();
}

fn main() {
    fn test(func: fn(s: String, word_dict: Vec<String>) -> bool) {
        assert_eq!(func(String::from("leetcode"), svec!["leet", "code"]), true);
        assert_eq!(func(String::from("applepenapple"), svec!["apple", "pen"]), true);
        assert_eq!(func(String::from("catsandog"), svec!["cats", "dog", "sand", "and", "cat"]), false);
    }
    test(word_break);
    test(word_break_dp);
    test(word_break_bfs);
}
