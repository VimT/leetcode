//! 使整数变为 0 的最少操作次数

/// 一个格雷码需要向零的方向枚举多少次才会变成0
pub fn minimum_one_bit_operations(n: i32) -> i32 {
    if n == 0 { return 0; }
    n ^ minimum_one_bit_operations(n >> 1)
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(3), 2);
        assert_eq!(func(6), 4);
    }
    test(minimum_one_bit_operations);
}
