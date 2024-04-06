//! 求出所有子序列的能量和

use std::collections::HashMap;

pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    const MOD: i32 = 1_000_000_007;
    nums.sort_unstable();

    // i: 当前遍历到哪个数，j: 剩余长度，pre: 前一个数，min_diff: 最小差值
    fn dfs(nums: &Vec<i32>, i: usize, j: usize, pre: i32, min_diff: i32, cache: &mut HashMap<(usize, usize, i32, i32), i32>) -> i32 {
        if j > i { return 0; }
        if j == 0 { return min_diff; }
        if let Some(&v) = cache.get(&(i, j, pre, min_diff)) {
            return v;
        }
        let mut result = 0;
        result += dfs(nums, i - 1, j, pre, min_diff, cache); // 不选
        result += dfs(nums, i - 1, j - 1, nums[i - 1], min_diff.min(pre - nums[i - 1]), cache); // 选
        result %= MOD;
        cache.insert((i, j, pre, min_diff), result);
        result
    }

    let mut cache = HashMap::new();
    dfs(&nums, len, k as usize, i32::MAX / 2, i32::MAX / 2, &mut cache)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4], 3), 4);
        assert_eq!(func(vec![2, 2], 2), 0);
        assert_eq!(func(vec![4, 3, -1], 2), 10);
    }
    test(sum_of_powers);
}
