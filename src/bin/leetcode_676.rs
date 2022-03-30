//! 实现一个魔法字典


use std::collections::HashMap;

use leetcode::svec;

struct MagicDictionary {
    m: HashMap<String, i32>,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            let mut w = word.into_bytes();
            let len = w.len();
            for i in 0..len {
                let ori = w[i];
                w[i] = b'*';
                unsafe { *self.m.entry(String::from_utf8_unchecked(w.clone())).or_default() += 1; }
                w[i] = ori;
            }
            unsafe { *self.m.entry(String::from_utf8_unchecked(w)).or_default() += 1; }
        }
    }

    fn search(&self, search_word: String) -> bool {
        let contain = self.m.contains_key(&search_word);
        let mut s = search_word.into_bytes();
        for i in 0..s.len() {
            let ori = s[i];
            s[i] = b'*';
            if !contain {
                unsafe {
                    if self.m.contains_key(std::str::from_utf8_unchecked(&s)) {
                        return true;
                    }
                }
            } else {
                unsafe {
                    if let Some(cnt) = self.m.get(std::str::from_utf8_unchecked(&s)) {
                        if *cnt > 1 {
                            return true;
                        }
                    }
                }
            }

            s[i] = ori;
        }
        false
    }
}


fn main() {
    let mut md = MagicDictionary::new();
    md.build_dict(svec!["hhllo", "hello", "leetcode"]);
    assert_eq!(md.search("hello".to_string()), true); // 返回 False
    assert_eq!(md.search("hhllo".to_string()), true); // 将第二个 'h' 替换为 'e' 可以匹配 "hello" ，所以返回 True
    assert_eq!(md.search("hell".to_string()), false); // 返回 False
    assert_eq!(md.search("leetcoded".to_string()), false); // 返回 False
}
