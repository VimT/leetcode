//! 相隔为 1 的编辑距离

pub fn is_one_edit_distance(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut i = 0;
    let len = s.len().min(t.len());
    while i < len && s[i] == t[i] {
        i += 1;
    }
    let mut j = i;
    if s.len() == t.len() {
        i += 1;
        j += 1;
    } else if s.len() + 1 == t.len() {
        j += 1;
    } else if s.len() == t.len() + 1 {
        i += 1;
    }
    while i < s.len() && j < t.len() && s[i] == t[j] {
        i += 1;
        j += 1;
    }
    i == s.len() && j == t.len()
}

fn main() {
    fn test(func: fn(s: String, t: String) -> bool) {
        assert_eq!(func(String::from("ab"), String::from("acb")), true);
        assert_eq!(func(String::from("cab"), String::from("ad")), false);
        assert_eq!(func(String::from(""), String::from("")), false);
    }
    test(is_one_edit_distance);
}
