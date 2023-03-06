//! 获得分数的方法数

/// 分组背包DP：先枚举体积，再枚举物品个数
pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = types.len();
    let target = target as usize;
    let mut dp = vec![vec![0; target + 1]; len + 1];
    dp[0][0] = 1;
    for i in 1..=len {
        for j in 0..=target {
            for k in 0..=(types[i - 1][0] as usize).min(j / types[i - 1][1] as usize) {
                dp[i][j] += dp[i - 1][j - k * types[i - 1][1] as usize];
            }
            dp[i][j] %= MOD;
        }
    }
    dp[len][target] as i32
}

/// 空间优化
pub fn ways_to_reach_target2(target: i32, types: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let target = target as usize;
    let mut dp = vec![0; target + 1];
    dp[0] = 1;
    for cm in types {
        let count = cm[0] as usize;
        let marks = cm[1] as usize;
        for j in (0..=target).rev() {
            for k in 1..=count.min(j / marks) { // k要从1开始，0的时候直接继承上一次的结果
                dp[j] += dp[j - k * marks];
            }
            dp[j] %= MOD;
        }
    }
    dp[target] as i32
}

fn main() {
    fn test(func: fn(target: i32, types: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(6, vec![vec![6, 1], vec![3, 2], vec![2, 3]]), 7);
        assert_eq!(func(5, vec![vec![50, 1], vec![50, 2], vec![50, 5]]), 4);
        assert_eq!(func(18, vec![vec![6, 1], vec![3, 2], vec![2, 3]]), 1);
    }
    test(ways_to_reach_target);
    test(ways_to_reach_target2);
}
