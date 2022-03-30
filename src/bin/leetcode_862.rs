//! 和至少为 K 的最短子数组

use std::collections::VecDeque;

/// 需要找到 x 和 y，使得 P[y] - P[x] >= K 且 y - x 最小
/// 滑动窗口, 窗口内就是x－y
/// 单调增队列
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as i64;
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut result = len + 1;
    let mut q = VecDeque::with_capacity(len + 1);
    for y in 0..len + 1 {
        // 为什么是单调增，新增的如果是下降趋势，那么如果前面作为x的满足 p[y] - p[x] >= k 那么后面的也肯定满足
        while !q.is_empty() && presum[y] <= presum[*q.back().unwrap()] {
            q.pop_back();
        }
        while !q.is_empty() && presum[y] >= presum[*q.front().unwrap()] + k {
            result = result.min(y - q.pop_front().unwrap());
        }
        q.push_back(y);
    }
    if result < len + 1 { result as i32 } else { -1 }
}

fn main() {
    assert_eq!(shortest_subarray(vec![1], 1), 1);
    assert_eq!(shortest_subarray(vec![1, 2], 4), -1);
    assert_eq!(shortest_subarray(vec![2, -1, 2], 3), 3);
}
