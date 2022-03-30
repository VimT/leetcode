//! 最长字符串链

use std::collections::HashMap;

use leetcode::svec;

pub fn longest_str_chain(words: Vec<String>) -> i32 {
    fn compare(a: &[u8], b: &[u8]) -> bool {
        let mut diff = false;
        let len = b.len();
        for i in 0..len {
            if diff {
                if a[i - 1] != b[i] { return false; }
            } else {
                if i < len - 1 && a[i] != b[i] { diff = true; }
            }
        }
        true
    }
    let words: Vec<&[u8]> = words.iter().map(|x| x.as_bytes()).collect();
    let mut len = vec![vec![]; 17];
    for i in 0..words.len() {
        len[words[i].len()].push(i);
    }
    let mut map = HashMap::new();
    for i in 1..16 {
        for &first in &len[i] {
            map.insert(first, vec![]);
            for &second in &len[i + 1] {
                if compare(words[first], words[second]) {
                    map.get_mut(&first).unwrap().push(second);
                }
            }
        }
    }
    let mut cache = HashMap::new();
    fn dfs(words: &Vec<&[u8]>, map: &HashMap<usize, Vec<usize>>, cur: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        let mut result = 1;
        if let Some(v) = cache.get(&cur) {
            return *v;
        }
        for next in map.get(&cur).unwrap_or(&vec![]) {
            result = result.max(dfs(words, map, *next, cache) + 1);
        }
        cache.insert(cur, result);
        result
    }
    let mut result = 1;
    for i in 1..16 {
        for &word in &len[i] {
            result = result.max(dfs(&words, &map, word, &mut cache));
        }
    }

    result
}

fn main() {
    assert_eq!(longest_str_chain(svec!["a", "b", "ba", "bca", "bda", "bdca"]), 4);
    assert_eq!(longest_str_chain(svec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]), 5);
    assert_eq!(longest_str_chain(svec!["abcd", "dbqca"]), 1);
}
