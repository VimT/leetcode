//! 单词接龙 II

use std::collections::{HashMap, HashSet, LinkedList};

use leetcode::svec;

/// 先BFS，形成树 和 最小距离。然后DFS 生成结果。
pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let mut h: HashMap<String, Vec<String>> = HashMap::new();
    let word_len = begin_word.len();
    let mut q = LinkedList::new();
    let mut visited = HashSet::new();

    if !word_list.contains(&end_word) {
        return vec![];
    }

    for string in word_list {
        for i in 0..word_len {
            let mut new = string.clone();
            new.replace_range(i..=i, "*");
            h.entry(new).or_insert(vec![]).push(string.clone());
        }
    }

    q.push_back(begin_word.clone());
    visited.insert(begin_word.clone());
    let mut adjacency = HashMap::new();
    let mut found = false;
    while !q.is_empty() {
        let mut next_level_visited = HashSet::new();
        let size = q.len();
        for _ in 0..size {
            let string = q.pop_front().unwrap();

            for i in 0..word_len {
                let mut new_string = string.clone();
                new_string.replace_range(i..=i, "*");
                for next_string in h.entry(new_string.clone()).or_default() {
                    if !visited.contains(next_string) {
                        if *next_string == end_word {
                            found = true;
                        }
                        if !next_level_visited.contains(next_string) {
                            q.push_back(next_string.clone());
                        }
                        next_level_visited.insert(next_string.clone());
                        adjacency.entry(string.clone()).or_insert(Vec::new()).push(next_string.clone());
                    }
                }
            }
        }
        if found { break; }
        visited.extend(next_level_visited);
    }
    let mut ans = vec![];
    fn dfs(first: &String, end: &String, adjacency: &HashMap<String, Vec<String>>, cur: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
        if first == end {
            ans.push(cur.clone());
            return;
        }
        if !adjacency.contains_key(first) { return; }
        for i in adjacency.get(first).unwrap() {
            cur.push(i.clone());
            dfs(i, end, adjacency, cur, ans);
            cur.pop();
        }
    }
    dfs(&begin_word, &end_word, &adjacency, &mut vec![begin_word.clone()], &mut ans);
    ans
}

fn main() {
    assert_eq!(find_ladders(String::from("hit"), String::from("cog"), svec!["hot", "dot", "dog", "lot", "log", "cog"]), vec![vec!["hit", "hot", "dot", "dog", "cog"], vec!["hit", "hot", "lot", "log", "cog"]]);
    assert_eq!(find_ladders(String::from("hit"), String::from("cog"), svec!["hot", "dot", "dog", "lot", "log"]).is_empty(), true);
}
