//! 最长合法子字符串的长度


use std::collections::HashSet;

/// trie
pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
    #[derive(Default, Debug)]
    pub struct Trie {
        children: [Option<Box<Trie>>; 26],
        is_word: bool,
    }

    impl Trie {
        pub fn insert(&mut self, s: &[u8]) {
            let mut node = self;
            for i in s.iter().rev() {
                let idx = (i - b'a') as usize;
                node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
            }
            node.is_word = true;
        }

        pub fn search(&self, s: &[u8]) -> Option<usize> {
            let mut node = self;
            for (&ch, i) in s.iter().rev().zip(0..) {
                let idx = (ch - b'a') as usize;
                if let Some(child) = &node.children[idx] {
                    node = child;
                    if child.is_word { return Some(i); }
                } else { break; }
            }
            None
        }
    }

    let mut trie = Trie::default();
    for w in forbidden {
        trie.insert(w.as_bytes());
    }
    let len = word.len();
    let mut l = 0;
    let mut result = 0;
    let w = word.as_bytes();
    for r in 0..len {
        if let Some(prev) = trie.search(&w[l..=r]) {
            l = r - prev + 1;
        }
        result = result.max(r + 1 - l);
    }
    result as i32
}

/// hash
pub fn longest_valid_substring2(word: String, forbidden: Vec<String>) -> i32 {
    let forbidden: HashSet<String> = forbidden.into_iter().collect();
    let len = word.len();
    let mut l = 0;
    let mut result = 0;
    for r in 0..len {
        for i in ((r as i32 - 10).max(l as i32) as usize..=r).rev() {
            if forbidden.contains(&word[i..=r]) {
                l = i + 1;
                break;
            }
        }
        result = result.max(r + 1 - l);
    }
    result as i32
}


fn main() {
    use leetcode::svec;
    fn test(func: fn(word: String, forbidden: Vec<String>) -> i32) {
        assert_eq!(func(String::from("cbaaaabc"), svec!["aaa","cb"]), 4);
        assert_eq!(func(String::from("leetcode"), svec!["de","le","e"]), 4);
    }
    test(longest_valid_substring);
    test(longest_valid_substring2);
}
