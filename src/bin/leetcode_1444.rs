//! 切披萨的方案数

pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
    let m = pizza.len();
    let n = pizza[0].len();
    let mut presum = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            presum[i + 1][j + 1] = presum[i][j + 1] + presum[i + 1][j] + (pizza[i].as_bytes()[j] == b'A') as i32 - presum[i][j];
        }
    }
    const MOD: i64 = 1e9 as i64 + 7;
    struct DFS {
        m: usize,
        n: usize,
        presum: Vec<Vec<i32>>,
        cache: Vec<Vec<Vec<i64>>>,
    }
    impl DFS {
        fn sum(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
            self.presum[x2][y2] + self.presum[x1][y1] - self.presum[x1][y2] - self.presum[x2][y1]
        }
        // 从(x,y)开始切k刀
        fn dfs(&mut self, x: usize, y: usize, k: usize) -> i64 {
            if self.cache[x][y][k] != -1 {
                return self.cache[x][y][k];
            }
            let mut result = 0;
            if k == 0 {
                result = (self.sum(x, y, self.m, self.n) > 0) as i64;
                self.cache[x][y][k] = result;
                return result;
            }
            for i in x + 1..self.m { // 横切
                if self.sum(x, y, i, self.n) > 0 {
                    result += self.dfs(i, y, k - 1);
                }
            }
            for j in y + 1..self.n {
                if self.sum(x, y, self.m, j) > 0 {
                    result += self.dfs(x, j, k - 1);
                }
            }
            result %= MOD;
            self.cache[x][y][k] = result;
            result
        }
    }
    let k = k as usize;
    let mut d = DFS { m, n, presum, cache: vec![vec![vec![-1; k]; n]; m] };
    d.dfs(0, 0, k - 1) as i32
}

/// 记忆化搜索转DP
pub fn ways2(pizza: Vec<String>, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let m = pizza.len();
    let n = pizza[0].len();
    let mut presum = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            presum[i + 1][j + 1] = presum[i][j + 1] + presum[i + 1][j] + (pizza[i].as_bytes()[j] == b'A') as i32 - presum[i][j];
        }
    }
    let sum = |x1: usize, y1: usize, x2: usize, y2: usize| -> i32 {
        presum[x2][y2] + presum[x1][y1] - presum[x1][y2] - presum[x2][y1]
    };
    let k = k as usize;
    let mut dp = vec![vec![vec![0; n]; m]; k + 1];
    for i in 0..m {
        for j in 0..n {
            if sum(i, j, m, n) > 0 {
                dp[0][i][j] = 1;
            }
        }
    }
    for k in 1..k {
        for x in (0..m).rev() {
            for y in (0..n).rev() {
                let mut tmp = 0;
                for i in x + 1..m { // 横切
                    if sum(x, y, i, n) > 0 {
                        tmp += dp[k - 1][i][y];
                    }
                }
                for j in y + 1..n {
                    if sum(x, y, m, j) > 0 {
                        tmp += dp[k - 1][x][j];
                    }
                }
                dp[k][x][y] = tmp % MOD;
            }
        }
    }
    dp[k - 1][0][0] as i32
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(pizza: Vec<String>, k: i32) -> i32) {
        assert_eq!(func(svec!["A..","AAA","..."], 3), 3);
        assert_eq!(func(svec!["A..","AA.","..."], 3), 1);
        assert_eq!(func(svec!["A..","A..","..."], 1), 1);
    }
    test(ways);
    test(ways2);
}
