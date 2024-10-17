//! 安排活动的方案数

use std::sync::OnceLock;

pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
    let (n, x, y) = (n as i64, x as i64, y as i64);
    const MOD: i64 = 1_000_000_007;
    const MX: usize = 1001;
    static S: OnceLock<Vec<Vec<i64>>> = OnceLock::new();
    let s = S.get_or_init(|| {
        let mut s = vec![vec![0; MX]; MX];
        s[0][0] = 1;
        for i in 1..MX {
            for j in 1..=i {
                s[i][j] = (s[i - 1][j - 1] + j as i64 * s[i - 1][j]) % MOD;
            }
        }
        s
    });
    let mut perm = 1;
    let mut pow_y = 1;
    let mut result = 0;
    for i in 1..=n.min(x) {
        perm = perm * (x + 1 - i) % MOD;
        pow_y = pow_y * y % MOD;
        result = (result + perm * s[n as usize][i as usize] % MOD * pow_y) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(n: i32, x: i32, y: i32) -> i32) {
        assert_eq!(func(1, 2, 3), 6);
        assert_eq!(func(5, 2, 1), 32);
        assert_eq!(func(3, 3, 4), 684);
    }
    test(number_of_ways);
}
