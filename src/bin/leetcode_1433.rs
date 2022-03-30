//! 检查一个字符串是否可以打破另一个字符串

pub fn check_if_can_break(s1: String, s2: String) -> bool {
    let mut s1 = s1.as_bytes().to_vec();
    let mut s2 = s2.as_bytes().to_vec();
    s1.sort_unstable();
    s2.sort_unstable();
    let len = s1.len();
    let mut f1 = true;
    let mut f2 = true;
    for i in 0..len {
        if s1[i] > s2[i] { f1 = false; }
        if s1[i] < s2[i] { f2 = false; }
    }
    f1 | f2
}

fn main() {
    assert_eq!(check_if_can_break("abc".to_string(), "xya".to_string()), true);
    assert_eq!(check_if_can_break("abe".to_string(), "acd".to_string()), false);
    assert_eq!(check_if_can_break("leetcodee".to_string(), "interview".to_string()), true);
}
