//! 网格图操作后的最大分数

/// 观察：黑柱子只会出现3种情况
/// 1. 持续递增
/// 2. 持续递减
/// 3. 递增后递减
/// 4. 递增/递减后 中间空一列。
pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    // 每一列的前缀和
    let mut col_presum = vec![vec![0; n + 1]; n + 1];
    for j in 0..n {
        for i in 0..n {
            col_presum[j + 1][i + 1] = col_presum[j + 1][i] + grid[i][j] as i64;
        }
    }

    let mut dp1 = vec![0; n + 1]; // 增长状态，当前涂黑 i 行的最大分数
    let mut dp2 = vec![i64::MIN; n + 1];  // 减少状态，当前涂黑 i 行的最大分数
    // 枚举每一列
    for j in 1..=n {
        let mut ndp1 = vec![0; n + 1];
        let mut ndp2 = vec![i64::MIN; n + 1];
        let mut cur = i64::MIN;
        for i in 0..=n {
            cur = cur.max(dp1[i] - col_presum[j - 1][i]);
            ndp1[i] = cur + col_presum[j - 1][i];
        }

        ndp1[0] = ndp1[0].max(dp2[0]); // 空了两列一此时这一行从0起步
        ndp1[n] = ndp1[n].max(dp2[0]); // 空了一列一此时这一行是n行起手，ndp[n]
        cur = i64::MIN;
        for i in (0..=n).rev() {
            cur = cur.max(dp2[i] + col_presum[j][i]);
            ndp2[i] = cur - col_presum[j][i];
        }

        // 递减情况：从满了的增长变为减少
        ndp2[n] = ndp2[n].max(ndp1[n]);
        dp1 = ndp1;
        dp2 = ndp2;
    }
    dp1.into_iter().chain(dp2).max().unwrap()
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![vec![0, 1, 9, 9, 8], vec![0, 0, 4, 0, 1], vec![6, 7, 0, 6, 0], vec![1, 1, 0, 14, 0], vec![14, 0, 15, 13, 13]]), 91);
        assert_eq!(func(vec![vec![11, 2, 0, 0], vec![2, 0, 0, 0], vec![13, 0, 0, 6], vec![0, 2, 0, 6]]), 40);
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 3, 0, 0], vec![0, 1, 0, 0, 0], vec![5, 0, 0, 3, 0], vec![0, 0, 0, 0, 2]]), 11);
        assert_eq!(func(vec![vec![10, 9, 0, 0, 15], vec![7, 1, 0, 8, 0], vec![5, 20, 0, 11, 0], vec![0, 0, 0, 1, 2], vec![8, 12, 1, 10, 3]]), 94);
    }
    test(maximum_score);
}
