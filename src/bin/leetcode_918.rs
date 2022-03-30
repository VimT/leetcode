//! 环形子数组的最大和


use std::collections::VecDeque;

/// baoli timeout
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = i32::MIN;
    for i in 0..len {
        let mut cur = 0;
        for j in i..i + len {
            cur += nums[j % len];
            result = result.max(cur);
        }
    }
    result
}

/// 算单个区间就用kadane，算跨区间 用 left_sum + right_sum_max
pub fn max_subarray_sum_circular2(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 { return nums[0]; }
    let mut result = nums[0];
    let mut dp = 0;
    for &num in &nums {
        dp = num + dp.max(0);
        result = result.max(dp);
    }
    let mut right_sum = vec![0; len];
    right_sum[len - 1] = nums[len - 1];
    for i in (0..len - 1).rev() {
        right_sum[i] = right_sum[i + 1] + nums[i];
    }
    let mut right_max = vec![0; len];
    right_max[len - 1] = nums[len - 1];
    for i in (0..len - 1).rev() {
        right_max[i] = right_max[i + 1].max(right_sum[i]);
    }
    let mut left_sum = 0;
    for i in 0..len - 2 {
        left_sum += nums[i];
        result = result.max(left_sum + right_max[i + 2]);
    }
    result
}

/// 前缀和 + 单调队列
/// 把数组×2，算前缀和，然后用严格单调增队列，算psum[cur] - psum队头
pub fn max_subarray_sum_circular3(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut p = vec![0; 2 * len + 1];
    for i in 0..2 * len {
        p[i + 1] = p[i] + nums[i % len];
    }
    let mut result = nums[0];
    let mut q = VecDeque::new();
    q.push_back(0);
    for j in 1..=2 * len {
        if *q.front().unwrap() + len < j {
            q.pop_front().unwrap();
        }
        result = result.max(p[j] - p[*q.front().unwrap()]);
        while !q.is_empty() && p[j] <= p[*q.back().unwrap()] {
            q.pop_back().unwrap();
        }
        q.push_back(j);
    }
    result
}

/// 对于跨区间情况，结果 = sum[0..i] + sum[j..len-1] = total_sum - (sum[i+1..j-1]) = total_sum + (-1 * (sum[i+1..j-1]))
/// total_sum是固定的，为了使后面的最大，即求kadane(A*-1)
/// 但是又不能直接求所有a翻转，考虑[-3,-2,-3]，全部翻转后再全加就是0。所以分两次求[1..len-1] [0..len-2]
pub fn max_subarray_sum_circular4(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 { return nums[0]; }
    let sum: i32 = nums.iter().sum();
    fn kadane(a: &Vec<i32>, i: usize, j: usize, sign: i32) -> i32 {
        let mut result = i32::MIN;
        let mut dp = i32::MIN;
        for k in i..j {
            dp = sign * a[k] + dp.max(0);
            result = result.max(dp);
        }
        result
    }
    let len = nums.len();
    let result1 = kadane(&nums, 0, len, 1);
    let result2 = sum + kadane(&nums, 1, len, -1);
    let result3 = sum + kadane(&nums, 0, len - 1, -1);
    result1.max(result2).max(result3)
}

/// Kadane 算法（最小值变种）
/// 用 min 代替 max。所有 Kadane 算法的解释依然相同，但是算法可以让我们找到最小子段和。
pub fn max_subarray_sum_circular5(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 { return nums[0]; }
    let sum: i32 = nums.iter().sum();
    let mut result1 = i32::MIN;
    let mut dp = i32::MIN;
    for &num in &nums {
        dp = num + dp.max(0);
        result1 = result1.max(dp);
    }
    let mut result2 = i32::MAX;
    dp = i32::MAX;
    for &num in &nums[1..] {
        dp = num + dp.min(0);
        result2 = result2.min(dp);
    }
    let mut result3 = i32::MAX;
    dp = i32::MAX;
    for &num in &nums[..nums.len() - 1] {
        dp = num + dp.min(0);
        result3 = result3.min(dp);
    }
    result1.max(sum - result2).max(sum - result3)
}

/// 对5的改进，不用算两遍
pub fn max_subarray_sum_circular6(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max_dp = 0;
    let mut max = nums[0];
    let mut min_dp = 0;
    let mut min = nums[0];
    for num in nums {
        sum += num;
        max_dp = num + max_dp.max(0);
        max = max.max(max_dp);
        min_dp = num + min_dp.min(0);
        min = min.min(min_dp);
    }
    if sum == min { return max; }
    max.max(sum - min)
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, -2, 3, -2]), 3);
        assert_eq!(func(vec![5, -3, 5]), 10);
        assert_eq!(func(vec![-3, -2, -3]), -2);
    }
    test(max_subarray_sum_circular);
    test(max_subarray_sum_circular2);
    test(max_subarray_sum_circular3);
    test(max_subarray_sum_circular4);
    test(max_subarray_sum_circular5);
    test(max_subarray_sum_circular6);
}
