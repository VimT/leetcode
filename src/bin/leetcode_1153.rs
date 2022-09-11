//! 字符串转化

pub fn can_convert(str1: String, str2: String) -> bool {
    if str1 == str2 { return true; }
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();
    let mut conv = vec![0; 128];
    let mut s2seen = [0; 26];
    for (&c1, &c2) in s1.iter().zip(s2.iter()) {
        if conv[c1 as usize] == 0 {
            conv[c1 as usize] = c2;
        } else if conv[c1 as usize] != c2 {
            return false;
        }
        s2seen[(c2 - b'a') as usize] = 1;
    }
    s2seen.iter().sum::<i32>() < 26
}

fn main() {
    fn test(func: fn(str1: String, str2: String) -> bool) {
        assert_eq!(func(String::from("abcdefghijklmnopqrstuvwxyz"), String::from("bcadefghijklmnopqrstuvwxzz")), true);
        assert_eq!(func(String::from("ab"), String::from("ba")), true);
        assert_eq!(func(String::from("aabcc"), String::from("ccdee")), true);
        assert_eq!(func(String::from("leetcode"), String::from("codeleet")), false);
    }
    test(can_convert);
}
