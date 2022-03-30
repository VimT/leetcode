//! 查找和替换模式

use leetcode::svec;

pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    let pattern = pattern.as_bytes();
    words.into_iter().filter(|word| {
        if word.len() != pattern.len() { return false; }
        let mut map1 = [0; 26];
        let mut map2 = [0; 26];
        let s = word.as_bytes();
        for (&w, &p) in s.iter().zip(pattern) {
            if map1[(w - b'a') as usize] == 0 && map2[(p - b'a') as usize] == 0 {
                map1[(w - b'a') as usize] = p;
                map2[(p - b'a') as usize] = w;
            } else {
                if map1[(w - b'a') as usize] != p || map2[(p - b'a') as usize] != w {
                    return false;
                }
            }
        }
        true
    }).collect()
}

fn main() {
    assert_eq!(find_and_replace_pattern(svec!["abc", "deq", "mee", "aqq", "dkd", "ccc"], String::from("abb")), svec!["mee", "aqq"]);
    assert_eq!(find_and_replace_pattern(svec!["a", "b", "c"], String::from("a")), svec!["a", "b", "c"]);
}
