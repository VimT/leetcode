//! 最高的广告牌

use std::collections::HashMap;

/// d[j] 高度差为 j 的时候，最长的公共长度
pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    dp.insert(0, 0);
    for rod in rods {
        for (d, y) in dp.clone() {
            dp.insert(d + rod, *dp.get(&(rod + d)).unwrap_or(&0).max(&y));
            dp.insert((d - rod).abs(), (*dp.get(&(d - rod).abs()).unwrap_or(&0)).max(y + d.min(rod)));
        }
    }
    dp[&0]
}

/// 定义状态方程：dp[i][j]: 表示用前i+1个钢筋，形成两个互斥子集合的差为j，子集合的和最大。
/// 例如【1，2，3，6】, 我们用前4个钢筋，差为1的两个子集合有很多，dp[3][1], 如{1}和{2}, {2}和{3}
/// {2,3}和{6}, 最大和的两个子集合，应该是{2,3}和{6}, 它们的和为11， 所以dp[3][1] = 11;
/// 我们的目标是找到dp[n][0]的值。
pub fn tallest_billboard_01(rods: Vec<i32>) -> i32 {
    let len = rods.len();
    if len < 2 {
        return 0;
    }
    let rods: Vec<usize> = rods.into_iter().map(|x| x as usize).collect();
    let sum: usize = rods.iter().sum();
    let mut dp = vec![vec![0; sum + 1]; len];
    //初始化，只有第1个钢筋的情况下，能够形成差为rods[0]的两个子集合，它们的和也是自身
    dp[0][rods[0]] = rods[0];
    let mut pre_sum = rods[0];
    for i in 1..len {
        for j in 0..=pre_sum {
            // 状态数组里有一些无效的值，如果两个子集的差为j, 它们的和肯定大于j。 这种dp值是无效的，不应该
            // 用来更新dp[i]的值
            if dp[i - 1][j] < j { continue; }

            // 更新dp[i][j], 有3种情况；
            // 1. 元素i不放入任一一个子集合， 那么差与和都不更新
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);

            // 2. 元素i放入和更大的子集合，那么子集合差就会更大，我们应该更新dp[i][j+rods[i]]
            let k = j + rods[i];
            dp[i][k] = dp[i][k].max(dp[i - 1][j] + rods[i]);

            // 3. 元素i放入和更小的子集合，那么子集合的差就会变小, 我们应该更新dp[i][j-rods[i]]
            let k = (j as i32 - rods[i] as i32).abs() as usize;
            dp[i][k] = dp[i][k].max(dp[i - 1][j] + rods[i]);
        }
        pre_sum += rods[i] as usize;
    }
    return (dp[len - 1][0] / 2) as i32;
}


fn main() {
    assert_eq!(tallest_billboard_01(vec![1, 2, 3, 6]), 6);
    assert_eq!(tallest_billboard_01(vec![1, 2, 3, 4, 5, 6]), 10);
    assert_eq!(tallest_billboard_01(vec![1, 2]), 0);
}
