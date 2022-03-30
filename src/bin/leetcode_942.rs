//! 增减字符串匹配

pub fn di_string_match(s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = Vec::with_capacity(len + 1);
    let mut lo = 0;
    let mut high = len as i32;
    for i in 0..len {
        if s[i] == b'I' {
            result.push(lo);
            lo += 1;
        } else {
            result.push(high);
            high -= 1;
        }
    }
    result.push(lo);
    result
}

fn main() {
    assert_eq!(di_string_match(String::from("IDID")), vec![0, 4, 1, 3, 2]);
    assert_eq!(di_string_match(String::from("III")), vec![0, 1, 2, 3]);
    assert_eq!(di_string_match(String::from("DDI")), vec![3, 2, 0, 1]);
}
