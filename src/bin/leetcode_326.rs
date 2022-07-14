//! 3 的幂

/// 在题目给定的 3232 位有符号整数的范围内，最大的 33 的幂为 3^19 = 11622614673
pub fn is_power_of_three(n: i32) -> bool {
    return n > 0 && 1162261467 % n == 0;
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(27), true);
        assert_eq!(func(0), false);
        assert_eq!(func(9), true);
    }
    test(is_power_of_three);
}
