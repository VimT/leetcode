//! 字符串的好分割数目

pub fn num_splits(s: String) -> i32 {
    let mut cnt = [false; 26];
    let s = s.as_bytes();
    let len = s.len();
    let mut suffix = vec![0; len + 1];
    let mut cur_cnt = 0;
    for i in (0..len).rev() {
        if !cnt[(s[i] - b'a') as usize] {
            cur_cnt += 1;
            cnt[(s[i] - b'a') as usize] = true;
        }
        suffix[i] = cur_cnt;
    }
    cnt.fill(false);
    cur_cnt = 0;
    let mut result = 0;
    for i in 0..len {
        if !cnt[(s[i] - b'a') as usize] {
            cur_cnt += 1;
            cnt[(s[i] - b'a') as usize] = true;
        }
        if cur_cnt == suffix[i + 1] {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("aacaba")), 2);
        assert_eq!(func(String::from("abcd")), 1);
        assert_eq!(func(String::from("aaaaa")), 4);
        assert_eq!(func(String::from("acbadbaada")), 2);
    }
    test(num_splits);
}
