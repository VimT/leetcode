//! 执行操作可获得的最大总奖励 I

/// 一定是 选最后一个数 + 前面 n-1 个数中选 和最大为 mx-1 的最大值
pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    // 前i个数，和最大为mx 的最大值
    fn dfs(nums: &Vec<i32>, i: usize, mx: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i == 0 || mx == 0 { return 0; }
        if mem[i][mx as usize] != -1 {
            return mem[i][mx as usize];
        }
        let mut result = dfs(nums, i - 1, mx, mem);
        if nums[i - 1] <= mx {
            result = result.max(nums[i - 1] + dfs(nums, i - 1, (mx - nums[i - 1]).min(nums[i - 1] - 1), mem));
        }
        mem[i][mx as usize] = result;
        result
    }
    reward_values.sort_unstable();
    let len = reward_values.len();
    let mx = reward_values.iter().max().copied().unwrap();
    let mut mem = vec![vec![-1; mx as usize + 1]; len + 1];
    mx + dfs(&reward_values, len - 1, mx - 1, &mut mem)
}

fn main() {
    fn test(func: fn(reward_values: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 3, 3]), 4);
        assert_eq!(func(vec![1, 6, 4, 3, 2]), 11);
    }
    test(max_total_reward);
}
