//! 滑动窗口最大值

use std::collections::{BinaryHeap, VecDeque};

/// 大顶堆做法，在大小为 k 的堆中插入一个元素消耗 log(k) 时间，因此算法的时间复杂度为 O(Nlog(k))
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let mut ans = vec![];

    for (i, &v) in nums.iter().enumerate() {
        heap.push((v, i as i32));
        loop {
            let &(max_value, max_value_index) = heap.peek().unwrap();
            if max_value_index + k > i as i32 {
                if i as i32 >= k - 1 { ans.push(max_value); }
                break;
            } else {
                heap.pop();
            }
        }
    }

    ans
}

/// 动态规划思路：dp1[i] 一个窗口所有左边数字的最大值，dp2[i]存这个窗口所有右边数字的最大值
pub fn max_sliding_window_dp(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut dp1 = nums.clone();
    let mut dp2 = nums.clone();
    let k = k as usize;
    for windows_nth in 0..nums.len() / k {
        for i in 1..k {
            let index = windows_nth * k + i;
            dp1[index] = dp1[index - 1].max(nums[index]);
        }
        for i in (0..k - 1).rev() {
            let index = windows_nth * k + i;
            dp2[index] = dp2[index + 1].max(nums[index]);
        }
    }
    let mut ans = vec![];

    for i in 0..=nums.len() - k {
        ans.push(dp1[i + k - 1].max(dp2[i]));
    }
    ans
}

/// 单调队列
/// 单调减队列，队头一直是最大值。
pub fn max_sliding_window_single_queue(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut q = VecDeque::with_capacity(nums.len());
    let mut ans = vec![];
    let k = k as usize;
    for i in 0..nums.len() {
        // pop掉过期的队头
        while !q.is_empty() && i >= k && *q.front().unwrap() <= i - k {
            q.pop_front();
        }
        // pop掉 不是单调减的队尾
        while !q.is_empty() && nums[*q.back().unwrap()] < nums[i] {
            q.pop_back();
        }
        q.push_back(i);
        if i >= k - 1 {
            ans.push(nums[*q.front().unwrap()]);
        }
    }
    ans
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
        assert_eq!(func(vec![1], 1), vec![1]);
    }
    test(max_sliding_window);
    test(max_sliding_window_dp);
    test(max_sliding_window_single_queue);
}
