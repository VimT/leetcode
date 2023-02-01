//! 猴子碰撞的方法数

use leetcode::algorithm::quick_pow;

pub fn monkey_move(n: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    ((quick_pow(2, n as i64, MOD) + MOD - 2) % MOD) as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(3), 6);
        assert_eq!(func(4), 14);
    }
    test(monkey_move);
}
