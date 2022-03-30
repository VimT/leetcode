//! 子串能表示从 1 到 N 数字的二进制串

/// 一部分是冗余的，只匹配N 到 (N>>1)+1，就可以，剩下的都已经包含在这里面了。
/// 举例，对于N=9，依次为1，10，11，100，101，110，111，1000，1001。可以看到k位长的后端包含了k-1位长的
pub fn query_string(s: String, n: i32) -> bool {
    for i in ((n >> 1) + 1..=n).rev() {
        if !s.contains(&format!("{:0b}", i)) {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(query_string(String::from("0110"), 3), true);
    assert_eq!(query_string(String::from("0110"), 4), false);
}
