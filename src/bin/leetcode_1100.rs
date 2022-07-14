//! 长度为 K 的无重复字符子串

pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
    let mut cnt = [0; 26];
    let s = s.as_bytes();
    for i in 0..k - 1 {
        cnt[(s[i as usize] - b'a') as usize] += 1;
    }
    let mut result = 0;
    for i in k as usize - 1..s.len() {
        cnt[(s[i] - b'a') as usize] += 1;
        let mut ok = true;
        for i in 0..26 {
            if cnt[i] > 1 {
                ok = false;
                break;
            }
        }
        if ok {
            result += 1;
        }
        cnt[(s[i + 1 - k as usize] - b'a') as usize] -= 1;
    }
    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("havefunonleetcode"), 5), 6);
        assert_eq!(func(String::from("home"), 5), 0);
    }
    test(num_k_len_substr_no_repeats);
}
