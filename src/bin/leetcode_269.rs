//! 火星词典

use std::collections::VecDeque;
use leetcode::svec;

/// 拓扑排序
pub fn alien_order(words: Vec<String>) -> String {
    let mut map = [[false; 26]; 26];
    for two_word in words.windows(2) {
        // 构建边
        let a = two_word[0].as_bytes();
        let b = two_word[1].as_bytes();
        let len = a.len().min(b.len());
        let mut diff = false;
        for i in 0..len {
            if a[i] != b[i] {
                map[(a[i] - b'a') as usize][(b[i] - b'a') as usize] = true;
                diff = true;
                break;
            }
        }
        if !diff && a.len() > b.len() {
            return String::new();
        }
    }
    let mut indegree = vec![-1; 26];
    for word in &words {
        for &ch in word.as_bytes() {
            indegree[(ch - b'a') as usize] = 0;
        }
    }
    let mut edge = vec![vec![]; 26];
    for i in 0..26 {
        for j in 0..26 {
            if map[i][j] {
                indegree[j] += 1;
                edge[i].push(j);
            }
        }
    }
    let mut q = VecDeque::new();
    for i in 0..26 {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }
    let mut result = vec![];
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        result.push(node as u8 + b'a');
        for &neigh in &edge[node] {
            indegree[neigh] -= 1;
            if indegree[neigh] == 0 {
                q.push_back(neigh);
            }
        }
    }
    for i in 0..26 {
        if indegree[i] > 0 {
            return String::new();
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> String) {
        assert_eq!(func(svec!["abc","ab"]), String::from(""));
        assert_eq!(func(svec!["wrt","wrf","er","ett","rftt"]), String::from("wertf"));
        assert_eq!(func(svec!["z","x"]), String::from("zx"));
        assert_eq!(func(svec!["z","x","z"]), String::from(""));
    }
    test(alien_order);
}
