//! 找出字符串的可整除数组

pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
    let s = word.as_bytes();
    let mut cur = 0;
    let m = m as i64;
    s.iter().map(|&ch| {
        cur = cur * 10 + (ch - b'0') as i64;
        cur %= m;
        if cur == 0 { 1 } else { 0 }
    }).collect()
}

fn main() {
    fn test(func: fn(word: String, m: i32) -> Vec<i32>) {
        assert_eq!(func(String::from("998244353"), 3), vec![1, 1, 0, 0, 0, 1, 1, 0, 0]);
        assert_eq!(func(String::from("1010"), 10), vec![0, 1, 0, 1]);
    }
    test(divisibility_array);
}
