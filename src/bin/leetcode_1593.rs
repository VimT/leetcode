//! 拆分字符串使唯一子字符串的数目最大

use std::collections::HashSet;

pub fn max_unique_split(s: String) -> i32 {
    fn dfs(s: &[u8], set: &mut HashSet<Vec<u8>>, result: &mut i32) {
        if s.is_empty() {
            *result = (*result).max(set.len() as i32);
            return;
        }

        for i in 1..=s.len() {
            if !set.contains(&s[..i]) {
                set.insert(s[..i].to_vec());
                dfs(&s[i..], set, result);
                set.remove(&s[..i]);
            }
        }
    }
    let mut result = 0;
    dfs(s.as_bytes(), &mut HashSet::new(), &mut result);
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("ababccc")), 5);
        assert_eq!(func(String::from("aba")), 2);
        assert_eq!(func(String::from("aa")), 1);
    }
    test(max_unique_split);
}
