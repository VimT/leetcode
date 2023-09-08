//! 将一个数字表示成幂的和的方案数

pub fn number_of_ways(n: i32, x: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;

    fn dfs(x: u32, n: i32, mx: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if n < 0 { return 0; } else if n == 0 { return 1; }
        if mx == 0 { return 0; }
        if mem[n as usize][mx as usize] != -1 {
            return mem[n as usize][mx as usize];
        }
        let mut result = 0;
        for i in (1..=mx).rev() {
            let num = i.pow(x);
            result = (result + dfs(x, n - num, i - 1, mem)) % MOD;
        }
        mem[n as usize][mx as usize] = result;
        result
    }
    let mx = (n as f64 + 1.).powf(1. / x as f64) as i32;
    dfs(x as u32, n, mx, &mut vec![vec![-1; mx as usize + 1]; n as usize + 1])
}

/// 把 n 看成背包容量，n^x 看成物品，本题就是一个 0-1 背包模板题
pub fn number_of_ways2(n: i32, x: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let mx = (n as f64 + 1.).powf(1. / x as f64) as usize;  // n + 1，防止 64^(1/3) = 2.99999
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; mx + 1]; // dp[i][j] 表示前i个num可以组成容量j的方案数
    for i in 1..=mx {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][j]
        }
    }
    let mut dp = vec![vec![0; n + 1]; mx + 1];
    dp[0][0] = 1;
    let x = x as u32;
    for i in 1..=mx {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][j];
            if j >= i.pow(x) {
                dp[i][j] = (dp[i][j] + dp[i - 1][j - i.pow(x)]) % MOD;
            }
        }
    }
    dp[mx][n]
}


/// dp空间优化
pub fn number_of_ways3(n: i32, x: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let mx = (n as f64 + 1.).powf(1. / x as f64) as usize;
    let n = n as usize;
    let x = x as u32;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for num in 1..=mx {
        for target in (0..=n).rev() {
            if target >= num.pow(x) {
                dp[target] = (dp[target] + dp[target - num.pow(x)]) % MOD;
            }
        }
    }
    dp[n]
}

/// dp预计算
pub fn number_of_ways4(n: i32, x: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    const MX_N: usize = 300;
    const MX_X: usize = 5;
    static mut DP: [[i32; MX_N + 1]; MX_X] = [[0; MX_N + 1]; MX_X];
    unsafe fn init() {
        for x in 0..MX_X {
            DP[x][0] = 1;
            for i in 1..=MX_N {
                let v = i.pow(x as u32 + 1);
                if v > MX_N { break; }
                for s in (v..=MX_N).rev() {
                    DP[x][s] = (DP[x][s] + DP[x][s - v]) % MOD;
                }
            }
        }
    }
    unsafe {
        if DP[0][0] == 0 { init(); }
        DP[x as usize - 1][n as usize] % MOD
    }
}

fn main() {
    fn test(func: fn(n: i32, x: i32) -> i32) {
        assert_eq!(func(64, 3), 1);
        assert_eq!(func(8, 3), 1);
        assert_eq!(func(1, 2), 1);
        assert_eq!(func(1, 1), 1);
        assert_eq!(func(10, 2), 1);
        assert_eq!(func(4, 1), 2);
    }
    test(number_of_ways);
    test(number_of_ways2);
    test(number_of_ways3);
    test(number_of_ways4);
}
