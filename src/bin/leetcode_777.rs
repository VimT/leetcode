//! 在LR字符串中交换相邻字符

pub fn can_transform(start: String, end: String) -> bool {
    let s = start.as_bytes();
    let e = end.as_bytes();
    if s.len() != e.len() { return false; }
    let len = s.len();
    let mut i = 0;
    let mut j = 0;
    while i < len || j < len {
        while i < len && s[i] == b'X' { i += 1; }
        while j < len && e[j] == b'X' { j += 1; }
        if (i < len) ^ (j < len) { return false; }
        if i < len && j < len {
            if s[i] != e[j] || (s[i] == b'L' && i < j) || (s[i] == b'R' && i > j) {
                return false;
            }
        }

        i += 1;
        j += 1;
    }
    true
}

fn main() {
    assert_eq!(can_transform(String::from("RXR"), String::from("XXR")), false);
    assert_eq!(can_transform(String::from("XXXXXLXXXX"), String::from("LXXXXXXXXX")), true);
    assert_eq!(can_transform(String::from("LXXLXRLXXL"), String::from("XLLXRXLXLX")), false);
    assert_eq!(can_transform(String::from("RXXLRXRXL"), String::from("XRLXXRRLX")), true);
    assert_eq!(can_transform(String::from("X"), String::from("L")), false);
}
