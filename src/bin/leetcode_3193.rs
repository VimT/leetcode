//! 统计逆序对的数目

pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1_000_000_007;

    // 前面i个数，逆序对为j时的排列个数
    fn dfs(req: &Vec<Option<usize>>, i: usize, j: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i == 0 { return 1; }
        if mem[i][j] != -1 { return mem[i][j]; }
        if let Some(r) = req[i - 1] {
            return if r <= j && j <= i + r { dfs(req, i - 1, r, mem) } else { 0 };
        }
        let mut result = 0;
        // 前面有k个数字比 perm[i] 大
        for k in 0..=j.min(i) {
            result += dfs(req, i - 1, j - k, mem);
            result %= MOD;
        }
        mem[i][j] = result;
        result
    }

    let n = n as usize;
    let mut req = vec![None; n];
    req[0] = Some(0);
    let mut m = 0;
    for r in requirements {
        req[r[0] as usize] = Some(r[1] as usize);
        m = m.max(r[1]);
    }
    if req[0].unwrap() != 0 { return 0; }
    let mut mem = vec![vec![-1; m as usize + 1]; n];
    dfs(&req, n - 1, req[n - 1].unwrap(), &mut mem)
}

/// dp
pub fn number_of_permutations2(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let mut req = vec![None; n];
    req[0] = Some(0);
    let mut m = 0;
    for r in requirements {
        req[r[0] as usize] = Some(r[1] as usize);
        m = m.max(r[1]);
    }
    if req[0].unwrap() != 0 { return 0; }
    let m = m as usize;
    let mut dp = vec![vec![0; m + 1]; n]; // dp[i][j] 表示前i个数，逆序对为j时的排列个数
    dp[0][0] = 1;
    for i in 1..n {
        if let Some(r) = req[i - 1] {
            for j in r..=(i + r).min(m) {
                dp[i][j] = dp[i - 1][r];
            }
        } else {
            for j in 0..=m {
                for k in 0..=j.min(i) {
                    dp[i][j] += dp[i - 1][j - k];
                    dp[i][j] %= MOD;
                }
            }
        }
    }
    dp[n - 1][req[n - 1].unwrap()]
}

/// dp优化
/// 1. dp[i][j] 的计算只和 dp[i-1][..] 有关，可以用滚动数组优化
/// 2. dp[i][j] = sum(dp[i-1][j-k] for k in 0..=j.min(i) ) 的过程可以用前缀和
pub fn number_of_permutations3(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let mut req = vec![None; n];
    req[0] = Some(0);
    let mut m = 0;
    for r in requirements {
        req[r[0] as usize] = Some(r[1] as usize);
        m = m.max(r[1]);
    }
    if req[0].unwrap() != 0 { return 0; }
    let m = m as usize;
    let mut dp = vec![0; m + 1];
    dp[0] = 1;
    for i in 1..n {
        if let Some(r) = req[i - 1] {
            for j in 0..=m {
                dp[j] = if r <= j && j <= i + r { dp[r] } else { 0 };
            }
        } else {
            for j in 1..=m {
                dp[j] = (dp[j] + dp[j - 1]) % MOD;
            }
            for j in (i + 1..=m).rev() {
                dp[j] = (dp[j] - dp[j - i - 1] + MOD) % MOD;
            }
        }
    }
    dp[req[n - 1].unwrap()]
}

fn main() {
    fn test(func: fn(n: i32, requirements: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![vec![2, 2], vec![0, 0]]), 2);
        assert_eq!(func(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]]), 1);
        assert_eq!(func(2, vec![vec![0, 0], vec![1, 0]]), 1);
    }
    test(number_of_permutations);
    test(number_of_permutations2);
    test(number_of_permutations3);
}
