//! 单词拆分 II

use std::collections::VecDeque;

use leetcode::{svec, unorder};

pub fn word_break_dp(s: &String, word_dict: &Vec<String>) -> bool {
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

/// 超时
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut dp = vec![vec![]; s.len() + 1];
    dp[0].push("".to_string());

    for i in 1..=s.len() {
        for start in 0..i {
            let substr = s[start..i].to_string();
            if dp[start].len() != 0 {
                if word_dict.contains(&substr) {
                    let mut strings = dp[start].iter().map(|x| [x.as_str(), " ", substr.as_str()].concat()).collect::<Vec<String>>();
                    dp[i].append(strings.as_mut());
                }
            }
        }
    }
    return dp.last().unwrap().iter().map(|x| x.trim().to_string()).collect::<Vec<String>>();
}

pub fn word_break_bfs(s: String, word_dict: Vec<String>) -> Vec<String> {
    if !word_break_dp(&s, &word_dict) {
        return vec![];
    }
    let mut ans = vec![];
    let mut queue: VecDeque<(usize, String)> = VecDeque::new();

    queue.push_back((0, "".to_string()));
    while !queue.is_empty() {
        let (start, current_str) = queue.pop_front().unwrap();

        if start == s.len() {
            ans.push(current_str);
            continue;
        }
        for end in start + 1..=s.len() {
            let substr = s[start..end].to_string();
            if word_dict.contains(&substr) {
                queue.push_back((end, [current_str.clone(), substr].join(" ").trim().to_string()));
            }
        }
    }
    ans
}


fn main() {
    fn test(func: fn(s: String, word_dict: Vec<String>) -> Vec<String>) {
        assert_eq!(unorder(func(String::from("catsanddog"), svec!["cat", "cats", "and", "sand", "dog"])), unorder(svec!["cats and dog", "cat sand dog"]));
        assert_eq!(unorder(func(String::from("pineapplepenapple"), svec!["apple", "pen", "applepen", "pine", "pineapple"])), unorder(svec!["pine apple pen apple", "pineapple pen apple", "pine applepen apple"]));
        assert_eq!(unorder(func(String::from("catsandog"), svec!["cats", "dog", "sand", "and", "cat"])), unorder(svec![]));
    }
    test(word_break);
    test(word_break_bfs);
}
