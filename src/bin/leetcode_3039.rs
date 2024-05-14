//! 进行操作使字符串为空

pub fn last_non_empty_string(s: String) -> String {
    let mut cnt = [0; 26];
    let mut last = [0; 26];
    let s = s.as_bytes();
    for (i, &ch) in s.iter().enumerate() {
        cnt[(ch - b'a') as usize] += 1;
        last[(ch - b'a') as usize] = i;
    }
    let mx = *cnt.iter().max().unwrap();
    let mut result: Vec<usize> = cnt.iter().enumerate().filter(|(_, &x)| x == mx).map(|(i, _)| last[i]).collect();
    result.sort_unstable();
    unsafe { String::from_utf8_unchecked(result.into_iter().map(|x| s[x]).collect()) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("aabcbbca")), String::from("ba"));
        assert_eq!(func(String::from("abcd")), String::from("abcd"));
    }
    test(last_non_empty_string);
}
