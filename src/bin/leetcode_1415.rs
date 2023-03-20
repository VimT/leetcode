//! 长度为 n 的开心字符串中字典序第 k 小的字符串

pub fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let mut s = vec![b'a'; n];
    fn dfs(s: &mut Vec<u8>, n: usize, k: usize, i: usize, result: &mut Vec<String>) {
        if result.len() >= k {
            return;
        }
        if i == n {
            result.push(unsafe { String::from_utf8_unchecked(s.clone()) });
            return;
        }
        for &ch in b"abc" {
            if i == 0 || s[i - 1] != ch {
                s[i] = ch;
                dfs(s, n, k, i + 1, result);
            }
        }
    }
    let mut result = Vec::with_capacity(k as usize + 2);
    dfs(&mut s, n, k as usize, 0, &mut result);
    result.get(k as usize - 1).cloned().unwrap_or_default()
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> String) {
        assert_eq!(func(1, 3), String::from("c"));
        assert_eq!(func(1, 4), String::from(""));
        assert_eq!(func(3, 9), String::from("cab"));
    }
    test(get_happy_string);
}
