//! 统计完全子字符串

pub fn count_complete_substrings(word: String, k: i32) -> i32 {
    let s = word.as_bytes();
    let mut result = 0;
    let len = s.len();
    let k = k as usize;
    let mut i = 0;
    while i < len {
        let st = i;
        i += 1;
        while i < len && (s[i] as i32 - s[i - 1] as i32).abs() <= 2 {
            i += 1;
        }
        for p in 1..=26 {
            let win = p * k;
            if i - st < win { break; }
            let mut cnt = vec![0; 26];
            for j in st..i {
                cnt[(s[j] - b'a') as usize] += 1;
                if j + 1 >= st + win {
                    if cnt.iter().all(|&v| v == 0 || v == k) { result += 1; }
                    cnt[(s[j + 1 - win] - b'a') as usize] -= 1;
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("gvgvvgv"), 2), 1);
        assert_eq!(func(String::from("rargaa"), 6), 0);
        assert_eq!(func(String::from("bba"), 1), 4);
        assert_eq!(func(String::from("baa"), 2), 1);
        assert_eq!(func(String::from("aca"), 1), 5);
        assert_eq!(func(String::from("abb"), 1), 4);
        assert_eq!(func(String::from("aaa"), 1), 3);
        assert_eq!(func(String::from("igigee"), 2), 3);
        assert_eq!(func(String::from("aaabbbccc"), 3), 6);
    }
    test(count_complete_substrings);
}
