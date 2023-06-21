//! 使数组中所有元素相等的最小操作数

pub fn min_operations(n: i32) -> i32 {
    if n & 1 == 1 {
        (n - 1 + 2) * (n - 1) / 4
    } else {
        (n - 1 + 1) * n / 4
    }
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(3), 2);
        assert_eq!(func(6), 9);
    }
    test(min_operations);
}
