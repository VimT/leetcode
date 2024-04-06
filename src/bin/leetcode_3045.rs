//! 统计前后缀下标对 II

use std::collections::HashMap;

pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
    #[derive(Clone, Default)]
    struct Trie {
        children: HashMap<(u8, u8), Box<Trie>>,
        cnt: i32,
    }
    impl Trie {
        fn insert(&mut self, arr: &[u8]) {
            let mut node = self;
            for (&a, &b) in arr.iter().zip(arr.iter().rev()) {
                node = node.children.entry((a, b)).or_default();
                node.cnt += 1;
            }
        }
        fn find_pre_suf_cnt(&self, arr: &[u8]) -> i32 {
            let mut node = self;
            for (&a, &b) in arr.iter().zip(arr.iter().rev()) {
                if let Some(next) = &node.children.get(&(a, b)) {
                    node = next;
                } else {
                    return 0;
                }
            }
            node.cnt
        }
    }

    let len = words.len();
    let mut result = 0;
    let mut trie = Trie::default();
    for i in (0..len).rev() {
        result += trie.find_pre_suf_cnt(words[i].as_bytes()) as i64;
        trie.insert(words[i].as_bytes());
    }
    result
}

/// Z函数：z[i] 表示后缀 t[i:] 与 t 的最长公共前缀的长度
pub fn count_prefix_suffix_pairs2(words: Vec<String>) -> i64 {
    #[derive(Clone, Default)]
    struct Trie {
        children: HashMap<u8, Box<Trie>>,
        cnt: i32,
    }

    fn calc_z(s: &[u8]) -> Vec<usize> {
        let len = s.len();
        let mut z = vec![0; len];
        let mut l = 0;
        let mut r = 0;
        for i in 1..len {
            if i <= r {
                z[i] = z[i - l].min(r + 1 - i);
            }
            while i + z[i] < len && s[z[i]] == s[i + z[i]] {
                (l, r) = (i, i + z[i]);
                z[i] += 1;
            }
        }
        z[0] = len;
        z
    }

    let mut result = 0;
    let mut trie = Trie::default();
    for t in words {
        let z = calc_z(t.as_bytes());
        let mut cur = &mut trie;
        for (i, &ch) in t.as_bytes().iter().enumerate() {
            cur = cur.children.entry(ch).or_default();
            if z[t.len() - 1 - i] == i + 1 {
                result += cur.cnt as i64;
            }
        }
        cur.cnt += 1;
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>) -> i64) {
        assert_eq!(func(svec!["a","aba","ababa","aa"]), 4);
        assert_eq!(func(svec!["pa","papa","ma","mama"]), 2);
        assert_eq!(func(svec!["abab","ab"]), 0);
    }
    test(count_prefix_suffix_pairs);
    test(count_prefix_suffix_pairs2);
}
