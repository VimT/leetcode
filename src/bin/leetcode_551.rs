//! 学生出勤记录 I

pub fn check_record(s: String) -> bool {
    let s = s.as_bytes();
    let mut start = 0;
    let len = s.len();
    let mut a = 0;
    while start < len {
        if s[start] == b'L' {
            let mut end = start + 1;
            while end < len && s[end] == b'L' {
                end += 1;
            }
            if end - start >= 3 { return false; }
            start = end;
        } else {
            if s[start] == b'A' { a += 1; }
            start += 1;
        }
    }
    a < 2
}

fn main() {
    assert_eq!(check_record(String::from("PPALLP")), true);
    assert_eq!(check_record(String::from("PPALLL")), false);
}
