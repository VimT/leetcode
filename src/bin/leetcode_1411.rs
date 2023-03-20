//! 给 N x 3 网格图涂色的方案数

pub fn num_of_ways(n: i32) -> i32 {
    let mut dp = [0; 27];
    for a in 0..3 {
        for b in 0..3 {
            if a != b {
                for c in 0..3 {
                    if b != c {
                        dp[a * 9 + b * 3 + c] = 1;
                    }
                }
            }
        }
    }
    const MOD: i64 = 1e9 as i64 + 7;
    for _ in 1..n {
        let mut new_dp = [0; 27];
        for a in 0..3 {
            for b in 0..3 {
                if a != b {
                    for c in 0..3 {
                        if b != c {
                            let idx = a * 9 + b * 3 + c;
                            for la in 0..3 {
                                if la != a {
                                    for lb in 0..3 {
                                        if lb != b && lb != la {
                                            for lc in 0..3 {
                                                if lc != c && lc != lb {
                                                    new_dp[idx] += dp[la * 9 + lb * 3 + lc];
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            new_dp[idx] %= MOD;
                        }
                    }
                }
            }
        }
        dp = new_dp;
    }
    (dp.iter().sum::<i64>() % MOD) as i32
}

/// 递推：发现颜色有两种类型：ABA，ABC，枚举上一行的类型和这一行的类型
pub fn num_of_ways2(n: i32) -> i32 {
    let mut f0 = 6;
    let mut f1 = 6;
    const MOD: i64 = 1e9 as i64 + 7;
    for _ in 2..=n {
        let nf0 = (2 * f0 + 2 * f1) % MOD;
        let nf1 = (2 * f0 + 3 * f1) % MOD;
        f0 = nf0;
        f1 = nf1;
    }
    ((f0 + f1) % MOD) as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(1), 12);
        assert_eq!(func(5000), 30228214);
    }
    test(num_of_ways);
    test(num_of_ways2);
}
