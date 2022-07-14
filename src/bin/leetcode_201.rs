//! 数字范围按位与

pub fn range_bitwise_and(m: i32, mut n: i32) -> i32 {
    while m < n {
        n = n & (n - 1);
    }
    n
}

fn main() {
    fn test(func: fn(left: i32, right: i32) -> i32) {
        assert_eq!(func(5, 7), 4);
        assert_eq!(func(0, 0), 0);
        assert_eq!(func(1, 2147483647), 0);
    }
    test(range_bitwise_and);
}
