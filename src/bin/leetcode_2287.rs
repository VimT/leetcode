//! 重排字符形成目标字符串

pub fn rearrange_characters(s: String, target: String) -> i32 {
    let mut cnt = [0; 26];
    for &ch in s.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut tcnt = [0; 26];
    for &ch in target.as_bytes() {
        tcnt[(ch - b'a') as usize] += 1;
    }
    let mut result = i32::MAX;
    for i in 0..26 {
        if tcnt[i] > 0 {
            result = result.min(cnt[i] / tcnt[i]);
        }
    }
    if result == i32::MAX { 0 } else { result }
}

fn main() {
    fn test(func: fn(s: String, target: String) -> i32) {
        assert_eq!(func(String::from("ilovecodingonleetcode"), String::from("code")), 2);
        assert_eq!(func(String::from("abcba"), String::from("abc")), 1);
        assert_eq!(func(String::from("abbaccaddaeea"), String::from("aaaaa")), 1);
    }
    test(rearrange_characters);
}
