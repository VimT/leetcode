//! 消除相邻近似相等字符

pub fn remove_almost_equal_characters(word: String) -> i32 {
    let s = word.as_bytes();
    let mut i = 0;
    let len = s.len();
    let mut result = 0;
    while i < len {
        let st = i;
        i += 1;
        while i < len && (s[i - 1] as i32 - s[i] as i32).abs() <= 1 {
            i += 1;
        }
        result += (i - st) / 2;
    }
    result as i32
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("aaaaa")), 2);
        assert_eq!(func(String::from("abddez")), 2);
        assert_eq!(func(String::from("zyxyxyz")), 3);
    }
    test(remove_almost_equal_characters);
}
