//! 字符串转换整数 (atoi)


pub fn my_atoi(str: String) -> i32 {
    if str.len() == 0 { return 0; }
    let chars = str.as_bytes();
    let mut ans: i64 = 0;
    let mut p = 0;
    let mut sign = 1;
    while p < chars.len() && chars[p].is_ascii_whitespace() { p += 1; }
    if p == chars.len() { return 0; }
    if chars[p] == b'-' || chars[p] == b'+' {
        sign = if chars[p] == b'-' { -1 } else { 1 };
        p += 1;
    }
    if p == chars.len() { return 0; }
    if !chars[p].is_ascii_digit() {
        return 0;
    }
    while p < chars.len() && chars[p].is_ascii_digit() {
        ans = ans * 10 + sign * (chars[p] - b'0') as i64;
        if ans < i32::MIN as i64 { return i32::MIN; }
        if ans > i32::MAX as i64 { return i32::MAX; }
        p += 1;
    }
    ans as i32
}


fn main() {
    assert_eq!(my_atoi(String::from("42")), 42);
    assert_eq!(my_atoi(String::from("   -42")), -42);
    assert_eq!(my_atoi(String::from("4193 with words")), 4193);
}
