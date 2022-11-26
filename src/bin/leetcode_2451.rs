//! 差值数组不同的字符串


use std::collections::HashMap;
use leetcode::svec;

pub fn odd_string(words: Vec<String>) -> String {
    let mut map: HashMap<Vec<i8>, Vec<String>> = HashMap::new();
    for word in words {
        let s = word.as_bytes();
        let mut diff = vec![0; s.len() - 1];
        for i in 0..s.len() - 1 {
            diff[i] = s[i + 1] as i8 - s[i] as i8;
        }
        map.entry(diff).or_default().push(word);
    }
    for (_, v) in map {
        if v.len() == 1 {
            return v[0].clone();
        }
    }
    unreachable!()
}


fn main() {
    assert_eq!(odd_string(svec!["adc","wzy","abc"]), "abc");
    assert_eq!(odd_string(svec!["aaa","bob","ccc","ddd"]), "bob");
}
