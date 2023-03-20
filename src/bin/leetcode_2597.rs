//! 美丽子集的数目

use std::collections::{BTreeMap, HashMap};

pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
    fn dfs(nums: &Vec<i32>, k: i32, i: usize, used: &mut Vec<i32>, result: &mut i32) {
        if i == nums.len() {
            *result += 1;
            return;
        }
        dfs(nums, k, i + 1, used, result);
        if k > nums[i] || used[(nums[i] - k) as usize] == 0 {
            used[nums[i] as usize] += 1;
            dfs(nums, k, i + 1, used, result);
            used[nums[i] as usize] -= 1;
        }
    }
    nums.sort_unstable();
    let mut result = 0;
    let mut used = vec![0; *nums.last().unwrap() as usize + 1];
    dfs(&nums, k, 0, &mut used, &mut result);
    result - 1
}


/// 动态规划
/// 1. 考虑k==1时，且nums[i]没有重复元素时，dp[i] 表示前i的数美丽子集数目
/// dp[i] = dp[i-1] + dp[i-2],  nums[i] - nums[i-1] == 1  // 选nums[i]，就是dp[i-2]，不选nums[i]，就是dp[i-1]个方案
/// dp[i] = dp[i-1] * 2,        nums[i] - nums[i-1] != 1
/// dp[0] = 1
/// 2. 考虑nums[i]有重复元素时，对nums[i]计数有cnt[nums[i]]个
/// dp[i] = dp[i-1] + dp[i-2] * 2.pow(cnt[i]),  nums[i] - nums[i-1] == 1
/// dp[i] = dp[i-1] * (2.pow(cnt[i]) - 1),            nums[i] - nums[i-1] != 1
/// 2. 考虑k==2时
/// 要把nums[i] 奇偶分组，按照乘法原理，结果是 odd_dp[-1] * even_dp[i-1]
/// 3. 考虑k=3时
/// 把nums[i]%3分成3组，再乘法原理
pub fn beautiful_subsets2(nums: Vec<i32>, k: i32) -> i32 {
    let max = *nums.iter().max().unwrap() as usize;
    let mut cnt = vec![0; max + 1];
    for num in nums {
        cnt[num as usize] += 1;
    }
    let mut dp_arr = vec![vec![]; k as usize];
    for i in 1..=max {
        if cnt[i] > 0 {
            dp_arr[i % k as usize].push(i as i32);
        }
    }
    fn dodp(arr: &Vec<i32>, cnt: &Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];
        dp[0] = 1;
        for i in 0..arr.len() {
            dp[i + 1] = if i == 0 || arr[i] - arr[i - 1] != k {
                dp[i] * (1 << cnt[arr[i] as usize])
            } else {
                dp[i] + dp[i - 1] * ((1 << cnt[arr[i] as usize]) - 1)
            }
        }
        dp[arr.len()]
    }
    let mut result = 1;
    for i in 0..k as usize {
        if !dp_arr[i].is_empty() {
            result *= dodp(&dp_arr[i], &cnt, k)
        }
    }
    result - 1
}

/// 动态规划的map版本
pub fn beautiful_subsets3(nums: Vec<i32>, k: i32) -> i32 {
    let mut m: HashMap<i32, BTreeMap<i32, i32>> = HashMap::new();
    for num in nums {
        *m.entry(num % k).or_default().entry(num).or_default() += 1;
    }
    let mut result = 1;
    for (_, group) in m {
        let mut dp = vec![0; group.len() + 1];
        dp[0] = 1;
        let mut last = 0;
        for (i, (num, cnt)) in group.into_iter().enumerate() {
            dp[i + 1] = if last == 0 || num - last != k {
                dp[i] * (1 << cnt)
            } else {
                dp[i] + dp[i - 1] * ((1 << cnt) - 1)
            };
            last = num;
        }
        result *= *dp.last().unwrap()
    }
    result - 1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1,2,3,3], 1), 8);
        assert_eq!(func(vec![10, 4, 5, 7, 2, 1], 3), 23);
        assert_eq!(func(vec![4, 2, 5, 9, 10, 3], 1), 23);
        assert_eq!(func(vec![2, 4, 6], 2), 4);
        assert_eq!(func(vec![1], 1), 1);
    }
    test(beautiful_subsets);
    test(beautiful_subsets2);
    test(beautiful_subsets3);
}
