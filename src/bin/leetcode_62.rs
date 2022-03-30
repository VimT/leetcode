//! 不同路径

pub fn unique_paths(m: i32, n: i32) -> i32 {
    fn inner(x: usize, y: usize, m: usize, n: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if x == m - 1 && y == n - 1 { return 1; }
        if cache[x][y] != -1 { return cache[x][y]; }
        let mut ans = 0;
        if x + 1 < m {
            ans += inner(x + 1, y, m, n, cache);
        }
        if y + 1 < n {
            ans += inner(x, y + 1, m, n, cache);
        }
        cache[x][y] = ans;
        ans
    }
    let m = m as usize;
    let n = n as usize;
    inner(0, 0, m, n, &mut vec![vec![-1; n]; m])
}

/// dp[i][j] 是到达 i, j 有多少条路径
/// dp[i][j] = dp[i-1][j] + dp[i][j-1]
pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; m + 1];
    dp[0][1] = 1;
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m][n]
}

pub fn unique_paths_dp_optimise(m: i32, n: i32) -> i32 {
    let mut dp = vec![1; n as usize];
    for _ in 1..m as usize {
        for j in 1..n as usize {
            dp[j] += dp[j - 1];
        }
    }
    return dp[n as usize - 1];
}

/// C(m-1, m+n-2)
pub fn unique_paths_math(m: i32, n: i32) -> i32 {
    fn factorial(i: i32) -> i32 {
        if i == 0 { return 1; }
        (1..=i).fold(1, |a, b| a * b)
    }
    factorial(m + n - 2) / factorial(m - 1) / factorial(n - 1)
}

fn main() {
    fn test(func: fn(m: i32, n: i32) -> i32) {
        assert_eq!(func(3, 7), 28);
        assert_eq!(func(3, 2), 3);
    }
    test(unique_paths);
    test(unique_paths_math);
    test(unique_paths_dp);
    test(unique_paths_dp_optimise);
}
