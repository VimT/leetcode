//! 公平分发饼干

pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    fn dfs(cookies: &Vec<i32>, idx: usize, cur: &mut Vec<i32>, result: &mut i32, mean: i32) {
        if idx == cookies.len() {
            *result = (*result).min(*cur.iter().max().unwrap());
            return;
        }
        for i in 0..cur.len() {
            if cur[i] > mean || cur[i] >= *result {
                continue;
            }
            cur[i] += cookies[idx];
            dfs(cookies, idx + 1, cur, result, mean);
            cur[i] -= cookies[idx];
        }
    }
    let mut result = i32::MAX;
    let sum: i32 = cookies.iter().sum();
    let mean = sum / k;
    dfs(&cookies, 0, &mut vec![0; k as usize], &mut result, mean);
    result
}

/// 子集状压DP，nb
/// 定义 f[i][j] 表示前 i 个孩子分配的饼干集合为 j 时，前 i 个孩子的不公平程度的最小值。
/// dp[i][j] = max( dp[i-1][j\s], sum[s] ) 给第i个孩子分配饼干集合s，sum[s]为元素和
pub fn distribute_cookies2(cookies: Vec<i32>, k: i32) -> i32 {
    let m = 1 << cookies.len();
    let mut sum = vec![0; m];
    for (i, &cookie) in cookies.iter().enumerate() {
        let bit = 1 << i;
        for mask in 0..1 << i {
            sum[bit | mask] = sum[mask] + cookie;
        }
    }
    let mut dp = sum.clone();
    for _ in 1..k {
        for j in (0..m).rev() {
            let mut s = j;
            while s > 0 {
                dp[j] = dp[j].min(dp[j ^ s].max(sum[s]));
                s = (s - 1) & j;
            }
        }
    }
    dp[m - 1]
}

fn main() {
    fn test(func: fn(cookies: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![8, 15, 10, 20, 8], 2), 31);
        assert_eq!(func(vec![6, 1, 3, 2, 2, 4, 1, 2], 3), 7);
    }
    test(distribute_cookies);
    test(distribute_cookies2);
}
