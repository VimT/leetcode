//! 最大异或乘积

/// 在把全0都变成全1之后。a^x + b^x 是一个定值
/// 均值定理：两个数的和是定值，那么乘积最大的时候，两个数相等
/// 实际的效果是 高位的1分配给一个数，其他1都分配给另一个数
pub fn maximum_xor_product(mut a: i64, mut b: i64, n: i32) -> i32 {
    for i in 0..n {
        let x = a >> i & 1;
        let y = b >> i & 1;
        if (x == 0 && y == 0) || (x != y && ((a ^ (1 << i)) - (b ^ (1 << i))).abs() < (a - b).abs()) {
            a ^= 1 << i;
            b ^= 1 << i;
        }
    }
    const MOD: i64 = 1_000_000_007;
    a %= MOD;
    b %= MOD;
    (a * b % MOD) as i32
}


fn main() {
    fn test(func: fn(a: i64, b: i64, n: i32) -> i32) {
        assert_eq!(func(12, 5, 4), 98);
        assert_eq!(func(6, 7, 5), 930);
        assert_eq!(func(1, 6, 3), 12);
    }
    test(maximum_xor_product);
}
