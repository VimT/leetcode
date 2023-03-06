//! 有效的快递序列数目

const MOD: usize = 1e9 as usize + 7;

/// 回溯思考
pub fn count_orders(n: i32) -> i32 {
    let n = n as usize;
    // i表示还有多少没有送，j表示还有多少没有收
    fn dfs(n: usize, i: usize, j: usize, dp: &mut Vec<Vec<Option<usize>>>) -> usize {
        if let Some(result) = dp[i][j] {
            return result;
        }
        let mut result =
            if j > 0 { dfs(n, i + 1, j - 1, dp) * j } else { 0 } +
                if i > 0 { dfs(n, i - 1, j, dp) * i } else { 0 };
        result %= MOD;
        dp[i][j] = Some(result);
        result
    }
    let dp = &mut vec![vec![None; n + 1]; n + 1];
    dp[0][0] = Some(1);
    dfs(n, 0, n, dp) as i32
}

/// 数学思考：考虑 (P1,D1,P2,D2) 序列，现在要插入 P3,D3，
/// 那么就是把(P3,D3)连续插入五个空位之一 C(5,1)
/// 或者分开插入5个空位的2个，就是C(5,2)
pub fn count_orders2(n: i32) -> i32 {
    let n = n as usize;
    let mut result = 1;
    for i in 2..=n {
        let space = (i - 1) * 2 + 1;
        result = (result * (space + space * (space - 1) / 2)) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(1), 1);
        assert_eq!(func(2), 6);
        assert_eq!(func(3), 90);
    }
    test(count_orders);
    test(count_orders2);
}
