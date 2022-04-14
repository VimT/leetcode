//! 加密解密字符串

use leetcode::svec;
use std::collections::HashMap;

struct Encrypter {
    enc: Vec<[u8; 2]>,
    dec: HashMap<String, i32>,
}


impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let len = keys.len();
        let mut enc = vec![[0; 2]; 26];
        for i in 0..len {
            let v = values[i].as_bytes();
            enc[(keys[i] as u8 - b'a') as usize] = [v[0], v[1]];
        }
        let mut result = Self { enc, dec: HashMap::new() };
        for word in dictionary {
            // 之前的用前缀树+dfs实现，不用这么复杂，直接加密计数就可以
            *result.dec.entry(result.encrypt(word)).or_default() += 1;
        }
        result
    }

    fn encrypt(&self, word1: String) -> String {
        let mut result = String::with_capacity(word1.len() * 2);
        for &ch in word1.as_bytes() {
            let x = self.enc[(ch - b'a') as usize];
            result.push(x[0] as char);
            result.push(x[1] as char);
        }
        result
    }

    fn decrypt(&self, word2: String) -> i32 {
        self.dec.get(&word2).cloned().unwrap_or(0)
    }
}

fn main() {
    let encrypter = Encrypter::new(vec!['a', 'b', 'c', 'd'], svec!["ei", "zf", "ei", "am"], svec!["abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"]);
    assert_eq!(encrypter.encrypt(String::from("abcd")), "eizfeiam");
    // "ei" 可以映射为 'a' 或 'c'，"zf" 映射为 'b'，"am" 映射为 'd'。
    // 因此，解密后可以得到的字符串是 "abad"，"cbad"，"abcd" 和 "cbcd"。
    // 其中 2 个字符串，"abad" 和 "abcd"，在 dictionary 中出现，所以答案是 2 。
    assert_eq!(encrypter.decrypt(String::from("eizfeiam")), 2);

    let encrypter = Encrypter::new(vec!['a', 'b', 'c', 'z'], svec!["aa", "bb", "cc", "zz"], svec!["aa","aaa","aaaa","aaaaa","aaaaaaa"]);
    assert_eq!(encrypter.decrypt(String::from("aa")), 0);
}
