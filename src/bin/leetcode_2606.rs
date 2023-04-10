//! 找到最大开销的子字符串

pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut m: Vec<i32> = (1..=26).collect();
    for (&ch, val) in chars.as_bytes().iter().zip(vals) {
        m[(ch - b'a') as usize] = val;
    }
    let mut cur = 0;
    for &ch in s.as_bytes() {
        cur += m[(ch - b'a') as usize];
        if cur < 0 {
            cur = 0;
        } else {
            result = result.max(cur);
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String, chars: String, vals: Vec<i32>) -> i32) {
        assert_eq!(func(String::from("adaa"), String::from("d"), vec![-1000]), 2);
        assert_eq!(func(String::from("abc"), String::from("abc"), vec![-1, -1, -1]), 0);
    }
    test(maximum_cost_substring);
}
