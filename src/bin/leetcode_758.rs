//! 字符串中的加粗单词

use leetcode::svec;


#[derive(Debug, Clone)]
pub struct Trie {
    children: Vec<Option<Box<Trie>>>,
    is_word: bool,
}

impl Default for Trie {
    fn default() -> Self {
        Self { children: vec![None; 128], is_word: false }
    }
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, s: &[u8]) {
        let mut node = self;
        for &i in s {
            node = node.children[i as usize].get_or_insert_with(Box::<Trie>::default);
        }
        node.is_word = true;
    }
}

/// same as 616
pub fn bold_words(words: Vec<String>, s: String) -> String {
    let mut trie = Trie::new();
    for word in words {
        trie.insert(word.as_bytes());
    }
    let s = s.as_bytes();
    let len = s.len();
    let mut is_bold = vec![false; len];
    for i in 0..len {
        let mut node = &trie;
        for j in i..len {
            if let Some(child) = &node.children[s[j] as usize] {
                node = child;
                if node.is_word {
                    for k in i..=j {
                        is_bold[k] = true;
                    }
                }
            } else {
                break;
            }
        }
    }
    let mut result = vec![];
    let mut i = 0;
    while i < len {
        if is_bold[i] {
            let mut j = i + 1;
            while j < len && is_bold[j] {
                j += 1;
            }
            result.extend_from_slice(b"<b>");
            result.extend_from_slice(&s[i..j]);
            result.extend_from_slice(b"</b>");
            i = j;
        } else {
            result.push(s[i]);
            i += 1;
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(words: Vec<String>, s: String) -> String) {
        assert_eq!(func(svec!["ab","bc"], String::from("aabcd")), String::from("a<b>abc</b>d"));
        assert_eq!(func(svec!["ab","cb"], String::from("aabcd")), String::from("a<b>ab</b>cd"));
    }
    test(bold_words);
}
