//! 每个元音包含偶数次的最长子字符串

pub fn find_the_longest_substring(s: String) -> i32 {
    let mut map = [None; 33];
    let mut result = 0;
    let mut cur = 0;
    map[0] = Some(0);
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        let idx = match ch {
            b'a' => 0,
            b'e' => 1,
            b'i' => 2,
            b'o' => 3,
            b'u' => 4,
            _ => 26,
        };
        if idx < 5 {
            cur ^= 1 << idx;
        }
        if let Some(j) = map[cur] {
            result = result.max(i + 1 - j);
        } else {
            map[cur] = Some(i + 1);
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("eleetminicoworoep")), 13);
        assert_eq!(func(String::from("leetcodeisgreat")), 5);
        assert_eq!(func(String::from("bcbcbc")), 6);
    }
    test(find_the_longest_substring);
}
