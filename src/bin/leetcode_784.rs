//! 字母大小写全排列

use leetcode::svec;

pub fn letter_case_permutation(s: String) -> Vec<String> {
    fn dfs(s: &mut Vec<u8>, mut cur: usize, result: &mut Vec<String>) {
        while cur < s.len() && s[cur].is_ascii_digit() { cur += 1; }
        if cur == s.len() {
            unsafe { result.push(String::from_utf8_unchecked(s.clone())); }
            return;
        }
        s[cur].make_ascii_lowercase();
        dfs(s, cur + 1, result);
        s[cur].make_ascii_uppercase();
        dfs(s, cur + 1, result);
    }
    let mut result = vec![];
    dfs(&mut s.into_bytes(), 0, &mut result);
    result
}

fn main() {
    assert_eq!(letter_case_permutation(String::from("a1b2")), svec!["a1b2", "a1B2", "A1b2", "A1B2"]);
    assert_eq!(letter_case_permutation(String::from("3z4")), svec!["3z4", "3Z4"]);
}
