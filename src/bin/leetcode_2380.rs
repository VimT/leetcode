//! 二进制字符串重新安排顺序需要的时间

pub fn seconds_to_remove_occurrences(s: String) -> i32 {
    let mut result = 0;
    let mut cnt = 0;
    for &ch in s.as_bytes() {
        if ch == b'0' {
            cnt += 1;
        } else if cnt > 0 {
            result = cnt.max(result + 1);
        }
    }

    result
}

fn main() {
    assert_eq!(seconds_to_remove_occurrences(String::from("0110101")), 4);
    assert_eq!(seconds_to_remove_occurrences(String::from("11100")), 0);
}