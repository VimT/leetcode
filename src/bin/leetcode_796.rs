//! 旋转字符串

pub fn str_str_kmp(haystack: String, needle: String) -> i32 {
    let query = haystack.as_bytes();
    let pattern = needle.as_bytes();
    let n = query.len();
    let m = pattern.len();
    if m == 0 { return 0; }
    let mut next = vec![0; m];
    let mut j = 0;
    for i in 1..m {
        // why while: aabaaa, last a need while
        while j > 0 && pattern[i] != pattern[j] {
            j = next[j - 1];
        }
        if pattern[i] == pattern[j] { j += 1; }
        next[i] = j;
    }
    j = 0;
    for i in 0..n {
        while j > 0 && query[i] != pattern[j] {
            j = next[j - 1];
        }
        if query[i] == pattern[j] { j += 1; }
        if j == m {
            return (i + 1 - m) as i32;
        }
    }
    -1
}

pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() { return false; }
    let s = s.clone() + &s;
    str_str_kmp(s, goal) != -1
}

fn main() {
    assert_eq!(rotate_string(String::from("abcde"), String::from("cdeab")), true);
    assert_eq!(rotate_string(String::from("abcde"), String::from("abced")), false);
}
