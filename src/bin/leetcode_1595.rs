//! 连通两组点的最小成本


/// dp[i][j] 表示左边组的前i的元素，匹配右边的集合为j时的最小代价。状态压缩，0表示已使用
/// 边界：dp[0][j] 表示对 j 集合的右边组元素，取 sum( minCost[k] )
pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
    let m = cost.len();
    let n = cost[0].len();

    let mut dp = vec![vec![i32::MAX; 1 << n]; m + 1];
    let min_cost: Vec<i32> = (0..n).map(|j| (0..m).map(|i| cost[i][j]).min().unwrap()).collect();
    dp[0].fill(0);
    for j in 0..1 << n {
        for k in 0..n {
            if j >> k & 1 == 0 {
                dp[0][j] += min_cost[k];
            }
        }
    }
    for i in 0..m {
        for j in 0..1 << n {
            for k in 0..n {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j | 1 << k] + cost[i][k]);
            }
        }
    }
    dp[m][0]
}

/// dp优化：
/// 1. 空间优化，降一个维度
/// 2. 初始值可以快速计算
pub fn connect_two_groups2(cost: Vec<Vec<i32>>) -> i32 {
    let m = cost.len();
    let n = cost[0].len();
    let mut dp = vec![0; 1 << n];
    for (j, mn) in (0..n).map(|j| (0..m).map(|i| cost[i][j]).min().unwrap()).enumerate() {
        let bit = 1 << j;
        for mask in 0..bit {
            dp[bit | mask] = dp[mask] + mn;
        }
    }
    for row in cost {
        for j in (0..1 << n).rev() {
            dp[j] = row.iter().enumerate().map(|(k, &c)| dp[j & !(1 << k)] + c).min().unwrap();
        }
    }
    *dp.last().unwrap()
}

/// 另一个dp转移，预处理出 左边每个点 对右边集合 的所有代价
pub fn connect_two_groups3(cost: Vec<Vec<i32>>) -> i32 {
    let m = cost.len();
    let n = cost[0].len();
    let mut cost_matrix = vec![vec![0; 1 << n]; m];
    for k in 0..m {
        for i in 0..1 << n {
            let mut sum = 0;
            for j in 0..n {
                if i & (1 << j) > 0 {
                    sum += cost[k][j];
                }
            }
            cost_matrix[k][i] = sum;
        }
    }
    let mut dp = vec![vec![i32::MAX; 1 << n]; m];
    dp[0] = cost_matrix[0].clone();
    for i in 1..m {
        for k in 1..1 << n {
            // 首先将第 i 行只选取一个元素的情况都考虑一遍
            // 这样做的目的是保证第 i 行至少选取了一个元素
            for j in 0..n {
                dp[i][k | (1 << j)] = dp[i][k | (1 << j)].min(dp[i - 1][k] + cost[i][j]);
            }
            // rest 表示截至第 i 行还没被选过的列
            let rest = (1 << n) - 1 - k;
            // 只遍历没选过的列的所有组合
            let mut j = rest;
            while j >= 1 {
                dp[i][j | k] = dp[i][j | k].min(dp[i - 1][k] + cost_matrix[i][j]);
                j = rest & (j - 1);
            }
        }
    }
    dp[m - 1][(1 << n) - 1]
}

fn main() {
    fn test(func: fn(cost: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![15, 96], vec![36, 2]]), 17);
        assert_eq!(func(vec![vec![7, 38], vec![43, 44], vec![72, 2], vec![64, 48], vec![90, 32], vec![10, 34], vec![50, 62], vec![99, 20], vec![39, 24]]), 236);
        assert_eq!(func(vec![vec![80, 96, 44], vec![38, 11, 8], vec![37, 73, 77], vec![77, 33, 57], vec![8, 72, 65], vec![48, 17, 66], vec![58, 62, 80], vec![70, 68, 39]]), 244);
        assert_eq!(func(vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]]), 4);
        assert_eq!(func(vec![vec![2, 5, 1], vec![3, 4, 7], vec![8, 1, 2], vec![6, 2, 4], vec![3, 8, 8]]), 10);
    }
    test(connect_two_groups);
    test(connect_two_groups2);
}

