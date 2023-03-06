//! 包含所有三种字符的子字符串数目

pub fn number_of_substrings(s: String) -> i32 {
    let mut cnt = [0; 3];
    let s = s.as_bytes();
    let len = s.len();
    let mut r = 0;
    let mut result = 0;
    for l in 0..len {
        while r < len && cnt.iter().any(|&x| x == 0) {
            cnt[(s[r] - b'a') as usize] += 1;
            r += 1;
        }
        if cnt.iter().all(|&x| x > 0) {
            result += len - r + 1;
        }
        cnt[(s[l] - b'a') as usize] -= 1;
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abcabc")), 10);
        assert_eq!(func(String::from("aaacb")), 3);
        assert_eq!(func(String::from("abc")), 1);
    }
    test(number_of_substrings);
}
