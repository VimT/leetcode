//! 带限制的子序列和

use std::collections::{BinaryHeap, VecDeque};

pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut result = nums[0];
    let k = k as usize;
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(len); // 过去k个数里最大的
    for i in 0..len {
        while !heap.is_empty() && heap.peek().unwrap().1 + k < i {
            heap.pop();
        }
        let x = heap.peek().map(|x| x.0).unwrap_or(0).max(0) + nums[i]; // 以i为结尾的前i+1个数的最大子序列和
        result = result.max(x);
        heap.push((x, i));
    }
    result
}

/// 单调队列,队头是前k个的最大值
pub fn constrained_subset_sum2(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as usize;
    let mut dp = vec![0; len];
    dp[0] = nums[0];
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut result = nums[0];
    for i in 1..len {
        while !q.is_empty() && i > k + *q.front().unwrap() {
            q.pop_front();
        }
        dp[i] = dp[*q.front().unwrap()].max(0) + nums[i];
        result = result.max(dp[i]);
        while !q.is_empty() && dp[i] >= dp[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![-5266, 4019, 7336, -3681, -5767], 2), 11355);
        assert_eq!(func(vec![10, 2, -10, 5, 20], 2), 37);
        assert_eq!(func(vec![-1, -2, -3], 1), -1);
        assert_eq!(func(vec![10, -2, -10, -5, 20], 2), 23);
    }
    test(constrained_subset_sum);
    test(constrained_subset_sum2);
}
