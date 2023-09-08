//! 判断通过操作能否让字符串相等 II

pub fn check_strings(s1: String, s2: String) -> bool {
    let mut a = [[0; 26]; 2];
    let mut b = [[0; 26]; 2];
    for (i, &ch) in s1.as_bytes().into_iter().enumerate() {
        a[i & 1][(ch - b'a') as usize] += 1;
    }
    for (i, &ch) in s2.as_bytes().into_iter().enumerate() {
        b[i & 1][(ch - b'a') as usize] += 1;
    }
    a == b
}

fn main() {
    fn test(func: fn(s1: String, s2: String) -> bool) {
        assert_eq!(func(String::from("abcdba"), String::from("cabdab")), true);
        assert_eq!(func(String::from("abe"), String::from("bea")), false);
    }
    test(check_strings);
}
