//! 删除字符串中的所有相邻重复项

pub fn remove_duplicates(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        if !result.is_empty() && *result.last().unwrap() == s[i] {
            result.pop();
        } else {
            result.push(s[i]);
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(remove_duplicates(String::from("abbaca")), "ca");
    assert_eq!(remove_duplicates(String::from("azxxzy")), "ay");
}
