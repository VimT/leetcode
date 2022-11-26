//! 掷骰子模拟

pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; 6]; n + 1];
    for j in 0..6 {
        dp[1][j] = 1;
    }
    const MOD: i64 = 1e9 as i64 + 7;
    for i in 2..=n {
        for j in 0..6 {
            let mut result: i64 = dp[i - 1].iter().sum();
            let idx = i as i32 - 1 - roll_max[j];
            if idx >= 1 {
                result = dp[idx as usize].iter().fold(result, |init, e| { init + MOD - *e });
                result += dp[idx as usize][j];
            } else if idx == 0 {
                result -= 1;
            }
            dp[i][j] = result % MOD;
        }
    }
    (dp[n].iter().sum::<i64>() % MOD) as i32
}

fn main() {
    fn test(func: fn(n: i32, roll_max: Vec<i32>) -> i32) {
        assert_eq!(func(2, vec![1, 1, 2, 2, 2, 3]), 34);
        assert_eq!(func(2, vec![1, 1, 1, 1, 1, 1]), 30);
        assert_eq!(func(3, vec![1, 1, 1, 2, 2, 3]), 181);
    }
    test(die_simulator);
}
