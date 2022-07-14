//! 花括号展开

use std::collections::VecDeque;

/// bfs 展开最里面的括号
pub fn expand(s: String) -> Vec<String> {
    let mut q = VecDeque::new();
    q.push_back(s);
    let mut result = vec![];
    while !q.is_empty() {
        let exp = q.pop_front().unwrap();
        if exp.find('{').is_none() {
            result.push(exp);
            continue;
        }
        let mut i = 0;
        let mut left = 0;
        let s = exp.as_bytes();
        while s[i] != b'}' {
            if s[i] == b'{' {
                left = i;
            }
            i += 1;
        }
        let right = i;

        let before = exp[0..left].to_string();
        let after = exp[right + 1..].to_string();
        let strs = exp[left + 1..right].split(',');
        for str in strs {
            q.push_back(format!("{}{}{}", before, str, after));
        }
    }
    result.sort_unstable();
    result
}

fn main() {
    fn test(func: fn(s: String) -> Vec<String>) {
        assert_eq!(func(String::from("{a,b}c{d,e}f")), vec!["acdf", "acef", "bcdf", "bcef"]);
        assert_eq!(func(String::from("abcd")), vec!["abcd"]);
    }
    test(expand);
}
