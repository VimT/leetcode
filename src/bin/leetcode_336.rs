//! 回文对


use std::collections::HashMap;

use leetcode::svec;

pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    #[derive(Default)]
    struct Node {
        ch: [usize; 26],
        flag: usize,
    }
    // 数组形式的trie树
    let mut trie = vec![Node::default()];

    fn insert(trie: &mut Vec<Node>, s: &[u8], idx: usize) {
        let mut add = 0;
        for &ch in s {
            let idx = (ch - b'a') as usize;
            if trie[add].ch[idx] == 0 {
                trie.push(Node::default());
                trie[add].ch[idx] = trie.len() - 1;
            }
            add = trie[add].ch[idx];
        }
        trie[add].flag = idx + 1;
    }

    fn find_word(trie: &Vec<Node>, s: &[u8], left: usize, right: usize) -> usize {
        let mut add = 0;
        for i in (left..right).rev() {
            let idx = (s[i] - b'a') as usize;
            if trie[add].ch[idx] == 0 {
                return 0;
            }
            add = trie[add].ch[idx];
        }
        trie[add].flag
    }

    #[inline]
    fn is_palindrome(s: &[u8], left: usize, right: usize) -> bool {
        if right < left {
            return false;
        }
        let len = right - left;
        for i in 0..len / 2 {
            if s[left + i] != s[right - i - 1] {
                return false;
            }
        }
        true
    }

    let len = words.len();
    for i in 0..len {
        insert(&mut trie, words[i].as_bytes(), i);
    }
    let mut result = vec![];
    for i in 0..len {
        let w = words[i].as_bytes();
        let m = w.len();
        for j in 0..=m {
            if is_palindrome(w, j, m) {
                let left_id = find_word(&trie, w, 0, j);
                if left_id > 0 && left_id != i + 1 {
                    result.push(vec![i as i32, left_id as i32 - 1]);
                }
            }
            if j > 0 && is_palindrome(w, 0, j) {
                let right_id = find_word(&trie, w, j, m);
                if right_id > 0 && i != right_id - 1 {
                    result.push(vec![right_id as i32 - 1, i as i32]);
                }
            }
        }
    }
    result
}

pub fn palindrome_pairs_set(words: Vec<String>) -> Vec<Vec<i32>> {
    let len = words.len();
    let mut map = HashMap::new();
    for i in 0..len {
        let mut s = words[i].as_bytes().to_vec();
        s.reverse();
        map.insert(s, i);
    }
    let mut result = vec![];

    #[inline]
    fn is_palindrome(s: &[u8]) -> bool {
        if s.len() < 2 { return true; }
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }

    for i in 0..len {
        let w = words[i].as_bytes();
        let m = w.len();
        for j in 0..=m {
            if is_palindrome(&w[j..m]) {
                if let Some(&left_id) = map.get(&w[..j]) {
                    if left_id != i {
                        result.push(vec![i as i32, left_id as i32]);
                    }
                }
            }
            if j > 0 && is_palindrome(&w[..j]) {
                if let Some(&right_id) = map.get(&w[j..m]) {
                    if right_id != i {
                        result.push(vec![right_id as i32, i as i32]);
                    }
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> Vec<Vec<i32>>) {
        assert_eq!(func(svec!["abcd", "dcba", "lls", "s", "sssll"]), vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]]);
        assert_eq!(func(svec!["bat", "tab", "cat"]), vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(func(svec!["a", ""]), vec![vec![0, 1], vec![1, 0]]);
    }
    test(palindrome_pairs);
    test(palindrome_pairs_set);
}
