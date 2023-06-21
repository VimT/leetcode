//! 找出第 N 个二进制字符串中的第 K 位

use std::cmp::Ordering;

pub fn find_kth_bit(n: i32, k: i32) -> char {
    fn get(n: i32, k: i32) -> u8 {
        if n == 1 { return 0; }
        let len = (1 << n) - 1;
        match k.cmp(&(len / 2)) {
            Ordering::Less => get(n - 1, k),
            Ordering::Equal => 1,
            Ordering::Greater => get(n - 1, len - 1 - k) ^ 1,
        }
    }
    (get(n, k - 1) + b'0') as char
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> char) {
        assert_eq!(func(4, 11), '1');
        assert_eq!(func(3, 1), '0');
        assert_eq!(func(1, 1), '0');
        assert_eq!(func(2, 3), '1');
    }
    test(find_kth_bit);
}
