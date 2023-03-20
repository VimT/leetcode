//! 生成数组


/// 记忆化搜索
pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    struct DFS {
        n: usize,
        m: usize,
        k: usize,
        cache: Vec<Vec<Vec<i32>>>,
    }
    impl DFS {
        // 当前长度，当前最大值，最大值更新次数
        fn dfs(&mut self, i: usize, j: usize, c: usize) -> i32 {
            if i == self.n { return (c == self.k) as i32; }
            if self.m - j < self.k - c { return 0; }
            if self.cache[i][j][c] != -1 { return self.cache[i][j][c]; }
            let mut result = j as i64 * self.dfs(i + 1, j, c) as i64 % MOD;
            if c < self.k {
                for x in j + 1..=self.m {
                    result += self.dfs(i + 1, x, c + 1) as i64;
                }
            }
            self.cache[i][j][c] = (result % MOD) as i32;
            self.cache[i][j][c]
        }
    }
    let mut d = DFS {
        n: n as usize,
        m: m as usize,
        k: k as usize,
        cache: vec![vec![vec![-1; k as usize + 1]; m as usize + 1]; n as usize + 1],
    };
    d.dfs(0, 0, 0)
}


/// dp + 前缀和优化 + 空间优化
pub fn num_of_arrays2(n: i32, m: i32, k: i32) -> i32 {
    const MOD: usize = 1e9 as usize + 7;
    if k == 0 { return 0; }
    let k = k as usize;
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; m + 1]; k + 1];
    dp[1][1..].fill(1);
    for i in 2..=n {
        for c in (1..=k.min(i)).rev() {
            let mut presum = 0;
            for j in 1..=m {
                dp[c][j] = dp[c][j] * j;
                dp[c][j] += presum;
                dp[c][j] %= MOD;
                presum += dp[c - 1][j];
            }
        }
    }
    (dp[k].iter().sum::<usize>() % MOD) as i32
}

fn main() {
    fn test(func: fn(n: i32, m: i32, k: i32) -> i32) {
        assert_eq!(func(2, 3, 1), 6);
        assert_eq!(func(5, 2, 3), 0);
        assert_eq!(func(9, 1, 1), 1);
        assert_eq!(func(50, 100, 25), 34549172);
        assert_eq!(func(37, 17, 7), 418930126);
    }
    test(num_of_arrays);
    test(num_of_arrays2);
}
