//! 执行操作可获得的最大总奖励 II

use leetcode::bitset::FixedBitSet;

/// 0-1背包，重量和容量是 rv[i]，容量是 2*max
pub fn max_total_reward0(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    let len = reward_values.len();
    let mx = reward_values[len - 1] as usize * 2;
    let mut dp = vec![false; mx]; // dp[i] 表示是否可以获得奖励值 i
    dp[0] = true;

    // 状态转移
    for &reward in &reward_values {
        let reward = reward as usize;
        for x in (reward..mx).rev() {
            if dp[x - reward] { dp[x] = true; }
        }
    }

    // 寻找最大可达的奖励值
    dp.iter().rposition(|&x| x).unwrap() as i32
}


/// bitset 优化的背包DP
pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    let len = reward_values.len();
    let mx = reward_values[len - 1];
    let mut x = FixedBitSet::new(mx as usize * 2);
    x.set(0);
    for num in reward_values {
        let mut y = x.clone();
        y.keep(num as usize);
        y.lsh(num as usize);
        x.or_assign(&y);
    }
    (mx..mx * 2).rfind(|&i| x.has(i as usize)).unwrap_or(mx)
}

/// dfs 剪枝
pub fn max_total_reward2(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    let len = reward_values.len();
    let mx = reward_values[len - 1];
    fn dfs(nums: &Vec<i32>, start: usize, max: i32, sum: i32, result: &mut i32, done: &mut bool) {
        if *done { return; }
        if sum == max - 1 {
            *result = max + sum;
            *done = true;
            return;
        }
        if sum >= max {
            *result = (*result).max(sum);
            return;
        }
        let nxt = nums[start..].binary_search_by(|x| x.cmp(&sum).then(std::cmp::Ordering::Less)).unwrap_err() + start;
        for i in nxt..nums.len() {
            dfs(nums, i + 1, max, sum + nums[i], result, done);
        }
    }
    let mut result = 0;
    dfs(&reward_values, 0, mx, 0, &mut result, &mut false);
    result
}

fn main() {
    fn test(func: fn(reward_values: Vec<i32>) -> i32) {
        assert_eq!(func(vec![49, 413, 190]), 652);
        assert_eq!(func(vec![1, 1, 3, 3]), 4);
        assert_eq!(func(vec![1, 6, 4, 3, 2]), 11);
    }
    test(max_total_reward);
    test(max_total_reward0);
    test(max_total_reward2);
}
