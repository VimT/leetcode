//! 相似度为 K 的字符串

use std::collections::{HashSet, VecDeque};

pub fn k_similarity(s1: String, s2: String) -> i32 {
    let s1 = s1.into_bytes();
    let s2 = s2.into_bytes();
    let mut q = VecDeque::new();
    #[inline]
    fn get_nxt(s: &Vec<u8>, target: &Vec<u8>) -> Vec<Vec<u8>> {
        let len = s.len();
        let mut result = vec![];
        for i in 0..len {
            if s[i] != target[i] {
                for j in i + 1..len {
                    if s[j] != target[j] && s[j] == target[i] {
                        let mut c = s.clone();
                        c.swap(i, j);
                        result.push(c);
                    }
                }
                break;
            }
        }
        result
    }
    let mut seen = HashSet::new();
    seen.insert(s1.clone());
    q.push_back((s1, 0));
    while !q.is_empty() {
        let (cur, step) = q.pop_front().unwrap();
        if cur == s2 {
            return step;
        }
        for nxt in get_nxt(&cur, &s2) {
            if !seen.contains(&nxt) {
                seen.insert(nxt.clone());
                q.push_back((nxt, step + 1));
            }
        }
    }
    0
}

fn main() {
    assert_eq!(k_similarity(String::from("cdebcdeadedaaaebfbcf"), String::from("baaddacfedebefdabecc")), 1);
    assert_eq!(k_similarity(String::from("ab"), String::from("ba")), 1);
    assert_eq!(k_similarity(String::from("abc"), String::from("bca")), 2);
}
