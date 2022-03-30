//! 字符串中的查找与替换

use leetcode::svec;

pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
    let s = s.as_bytes();
    let mut result = Vec::with_capacity(s.len() * 2);
    let mut last = 0;
    let mut idx: Vec<(usize, usize)> = indices.into_iter().enumerate().map(|(i, v)| (v as usize, i)).collect();
    idx.sort_unstable();
    for (start, i) in idx {
        let source = sources[i].as_bytes();
        let target = targets[i].as_bytes();
        let end = start + source.len();
        if &s[start..end] == source {
            result.extend_from_slice(&s[last..start]);
            result.extend_from_slice(target);
            last = end;
        }
    }
    result.extend_from_slice(&s[last..]);
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(find_replace_string("vmokgggqzp".to_string(), vec![3, 5, 1], svec!["kg", "ggq", "mo"], svec!["s", "so", "bfr"]), "vbfrssozp");
    assert_eq!(find_replace_string("abcd".to_string(), vec![0, 2], svec!["a", "cd"], svec!["eee", "ffff"]), "eeebffff");
    assert_eq!(find_replace_string("abcd".to_string(), vec![0, 2], svec!["ab", "ec"], svec!["eee", "ffff"]), "eeecd");
}
