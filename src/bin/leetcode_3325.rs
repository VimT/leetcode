//! 字符至少出现 K 次的子字符串 I

pub fn number_of_substrings(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    let mut morek = 0;
    let mut result = 0;
    let len = s.len();
    let mut r = 0;
    for l in 0..len {
        while r < len && morek == 0 {
            cnt[(s[r] - b'a') as usize] += 1;
            if cnt[(s[r] - b'a') as usize] == k { morek += 1; }
            r += 1;
        }
        if morek > 0 { result += len + 1 - r; }
        cnt[(s[l] - b'a') as usize] -= 1;
        if cnt[(s[l] - b'a') as usize] == k - 1 {
            morek -= 1;
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("abacb"), 2), 4);
        assert_eq!(func(String::from("abcde"), 1), 15);
    }
    test(number_of_substrings);
}
