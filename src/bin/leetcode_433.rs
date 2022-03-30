//! 最小基因变化

use std::collections::VecDeque;

use leetcode::svec;

pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    let len = bank.len();
    if len == 0 { return -1; }
    #[inline]
    fn diff(a: &[u8], b: &[u8]) -> u8 {
        let mut result = 0;
        for i in 0..a.len() {
            if a[i] != b[i] { result += 1; }
        }
        result
    }
    let mut m = vec![vec![]; len + 1];
    let mut end_idx = 0;
    for i in 0..len {
        if end == bank[i] { end_idx = i; }
        for j in i + 1..len {
            if diff(bank[i].as_bytes(), bank[j].as_bytes()) == 1 {
                m[i].push(j);
                m[j].push(i);
            }
        }
        if diff(bank[i].as_bytes(), start.as_bytes()) == 1 {
            m[len].push(i);
        }
    }
    if bank[end_idx] != end { return -1; }
    let mut q = VecDeque::new();
    q.push_back((len, 0));
    let mut vis = vec![false; len + 1];
    vis[len] = true;
    while !q.is_empty() {
        let (cur, step) = q.pop_front().unwrap();
        for &nxt in &m[cur] {
            if nxt == end_idx { return step + 1; }
            if !vis[nxt] {
                vis[nxt] = true;
                q.push_back((nxt, step + 1));
            }
        }
    }

    -1
}

fn main() {
    assert_eq!(min_mutation(String::from("AACCGGTT"), String::from("AACCGGTA"), svec!["AACCGGTA"]), 1);
    assert_eq!(min_mutation(String::from("AACCGGTT"), String::from("AAACGGTA"), svec!["AACCGGTA", "AACCGCTA", "AAACGGTA"]), 2);
    assert_eq!(min_mutation(String::from("AAAAACCC"), String::from("AACCCCCC"), svec!["AAAACCCC", "AAACCCCC", "AACCCCCC"]), 3);
}
