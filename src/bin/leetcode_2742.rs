//! 给墙壁刷油漆

pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let ct: Vec<(i32, i32)> = cost.into_iter().zip(time).collect();
    let len = ct.len();
    fn dfs(ct: &Vec<(i32, i32)>, i: usize, can_do: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i + can_do - ct.len() >= ct.len() {
            return 0;
        }
        if i == ct.len() {
            return if can_do >= ct.len() {
                0
            } else {
                i32::MAX / 2
            };
        }
        if mem[i][can_do] != -1 {
            return mem[i][can_do];
        }
        let (cost, time) = ct[i];
        let mut result = dfs(ct, i + 1, can_do - 1, mem);
        result = result.min(cost + dfs(ct, i + 1, can_do + time as usize, mem));
        mem[i][can_do] = result;
        result
    }

    dfs(&ct, 0, len, &mut vec![vec![-1; len * 2]; len])
}


/// dp[i][len+j] 表示粉刷前i个墙之后，有 j 个多余的时间，所消耗的最小代价
pub fn paint_walls2(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let len = cost.len();
    let mut dp = vec![vec![i32::MAX / 2; len * 2 + 1]; len + 1];
    dp[0][len] = 0;
    for i in 0..len {
        for j in 0..=len * 2 {
            if j > 0 {
                dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j]);
            }
            dp[i + 1][(j + time[i] as usize).min(2 * len)] = dp[i + 1][(j + time[i] as usize).min(2 * len)].min(dp[i][j] + cost[i]);
        }
    }
    dp[len][len..].iter().min().copied().unwrap()
}


/// 0-1背包：
/// 付费墙数量 + 免费墙数量 = n
/// 付费墙时间 >= 免费墙数量
/// 付费墙时间 >= n - 付费墙数量
/// 付费墙时间 + 付费墙数量 >= n
/// (time[i1] + 1) + (time[i2] + 1) + ... + (time[in] + 1) >= n
/// 这样转化成 0-1 背包问题，选一些墙付费墙，sum(time[i] + 1) >= n，且 sum(cost[i]) 最小
/// 即：体积： time[i]+1，价值：cost[i]。求最小化价值和
/// dp[i][j] 表示前i个物品，体积和至少是j的时候，最小的价值和
pub fn paint_walls3(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let len = cost.len();
    let mut dp = vec![i32::MAX / 2; len + 1];
    dp[0] = 0;
    for (c, t) in cost.into_iter().zip(time) {
        for j in (1..=len).rev() {
            dp[j] = dp[j].min(dp[(j as i32 - t - 1).max(0) as usize] + c);
        }
    }
    dp[len]
}

fn main() {
    fn test(func: fn(cost: Vec<i32>, time: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 8, 9, 13], vec![2, 2, 1, 2]), 9);
        assert_eq!(func(vec![7, 15, 38, 35, 61, 90, 34, 29, 68, 35], vec![1, 1, 3, 3, 2, 1, 3, 1, 2, 3]), 76);
        assert_eq!(func(vec![26, 53, 10, 24, 25, 20, 63, 51], vec![1, 1, 1, 1, 2, 2, 2, 1]), 55);
        assert_eq!(func(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
        assert_eq!(func(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }
    test(paint_walls);
    test(paint_walls2);
    test(paint_walls3);
}
