//! 自定义字符串排序

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut m = [50; 26];
    let order = order.as_bytes();
    for (i, &ch) in order.iter().enumerate() {
        m[(ch - b'a') as usize] = i;
    }
    let mut s = s.into_bytes();
    s.sort_unstable_by_key(|&x| m[(x - b'a') as usize]);
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(custom_sort_string(String::from("cba"), String::from("abcd")), String::from("cbad"));
    assert_eq!(custom_sort_string(String::from("cbafg"), String::from("abcd")), String::from("cbad"));
}
