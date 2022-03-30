//! 面试题 16.18. 模式匹配
pub fn pattern_matching(pattern: String, value: String) -> bool {
    let pattern = pattern.as_bytes();
    let a_count = pattern.iter().filter(|x| **x == b'a').count();
    let b_count = pattern.len() - a_count;
    let value_len = value.len();
    if a_count == 0 && b_count == 0 { return value == ""; }
    if a_count == 0 { return value == value[0..value_len / b_count].repeat(b_count); }
    if b_count == 0 { return value == value[0..value_len / a_count].repeat(a_count); }
    let mut a = "";
    let mut b = "";
    for a_len in 0..=value_len / a_count {
        let b_total_len = value_len - a_len * a_count;
        if b_total_len % b_count != 0 { continue; }
        let b_len = b_total_len / b_count;
        let mut vi = 0;
        for i in 0..pattern.len() {
            if pattern[i] == b'a' {
                if a.len() != a_len {
                    a = &value[vi..vi + a_len];
                } else {
                    if a != &value[vi..vi + a_len] { break; }
                }
                vi += a_len;
            }
            if pattern[i] == b'b' {
                if b.len() != b_len {
                    b = &value[vi..vi + b_len];
                } else {
                    if b != &value[vi..vi + b_len] { break; }
                }
                vi += b_len;
            }
        }
        if vi == value_len && a != b { return true; }
    }

    false
}

fn main() {
    assert!(!pattern_matching("ab".to_string(), "".to_string()));
    assert!(pattern_matching("abba".to_string(), "dogcatcatdog".to_string()));
    assert!(!pattern_matching("abba".to_string(), "dogcatcatfish".to_string()));
    assert!(!pattern_matching("aaaa".to_string(), "dogcatcatdog".to_string()));
    assert!(pattern_matching("abba".to_string(), "dogdogdogdog".to_string()));
    assert!(pattern_matching("".to_string(), "".to_string()));
}
