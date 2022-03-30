//! 单词接龙

use std::collections::{HashMap, HashSet, LinkedList};

use leetcode::svec;

/// bfs
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut h: HashMap<String, Vec<String>> = HashMap::new();
    let word_len = begin_word.len();
    let mut q = LinkedList::new();
    let mut visited = HashSet::new();

    if !word_list.contains(&end_word) {
        return 0;
    }

    for string in word_list {
        for i in 0..word_len {
            let new: String = [&string[0..i], "*", &string[i + 1..]].concat();
            h.entry(new).or_insert(vec![]).push(string.clone());
        }
    }

    q.push_back((begin_word, 1));

    while !q.is_empty() {
        let (string, level) = q.pop_front().unwrap();

        for i in 0..word_len {
            let new_string = [&string[0..i], "*", &string[i + 1..]].concat();
            for next_string in h.entry(new_string).or_default() {
                if *next_string == end_word {
                    return level + 1;
                }
                if !visited.contains(next_string) {
                    visited.insert(next_string.clone());
                    q.push_back((next_string.clone(), level + 1));
                }
            }
        }
    }
    0
}

pub fn ladder_length_both_bfs(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut h: HashMap<String, Vec<String>> = HashMap::new();
    let word_len = begin_word.len();
    let mut q_start = LinkedList::new();
    let mut q_end = LinkedList::new();
    let mut start_visited = HashMap::new();
    let mut end_visited = HashMap::new();

    if !word_list.contains(&end_word) {
        return 0;
    }

    for string in word_list {
        for i in 0..word_len {
            let new: String = [&string[0..i], "*", &string[i + 1..]].concat();
            h.entry(new).or_insert(vec![]).push(string.clone());
        }
    }

    q_start.push_back((begin_word.clone(), 1));
    q_end.push_back((end_word.clone(), 1));
    start_visited.insert(begin_word, 1);
    end_visited.insert(end_word, 1);

    /// 每次遍历一层节点
    fn bfs(h: &HashMap<String, Vec<String>>, queue: &mut LinkedList<(String, i32)>, visited: &mut HashMap<String, i32>, other_visited: &HashMap<String, i32>) -> i32 {
        let (string, level) = queue.pop_front().unwrap();
        for i in 0..string.len() {
            let new_string = [&string[0..i], "*", &string[i + 1..]].concat();
            for next_string in h.get(&new_string).unwrap_or(&vec![]) {
                if other_visited.contains_key(next_string) {
                    return level + other_visited[next_string];
                }
                if !visited.contains_key(next_string) {
                    visited.insert(next_string.clone(), level + 1);
                    queue.push_back((next_string.clone(), level + 1));
                }
            }
        }
        0
    }

    while !q_start.is_empty() && !q_end.is_empty() {
        let ans = bfs(&h, &mut q_start, &mut start_visited, &end_visited);
        if ans != 0 { return ans; }
        let ans = bfs(&h, &mut q_end, &mut end_visited, &start_visited);
        if ans != 0 { return ans; }
    }
    0
}

fn main() {
    fn test(func: fn(begin_word: String, end_word: String, word_list: Vec<String>) -> i32) {
        assert_eq!(func(String::from("hit"), String::from("cog"), svec!["hot", "dot", "dog", "lot", "log", "cog"]), 5);
        assert_eq!(func(String::from("hit"), String::from("cog"), svec!["hot", "dot", "dog", "lot", "log"]), 0);
    }
    test(ladder_length);
    test(ladder_length_both_bfs);
}
