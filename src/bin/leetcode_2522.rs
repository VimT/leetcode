//! 将字符串分割成值不超过 K 的子字符串

/// 贪心
pub fn minimum_partition(s: String, k: i32) -> i32 {
    let mut result = 1;
    let mut x = 0;
    let k = k as i64;
    for &ch in s.as_bytes() {
        let v = (ch - b'0') as i64;
        if v > k { return -1; }
        x = x * 10 + v;
        if x > k {
            result += 1;
            x = v;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("165462"), 60), 4);
        assert_eq!(func(String::from("238182"), 5), -1);
    }
    test(minimum_partition);
}
