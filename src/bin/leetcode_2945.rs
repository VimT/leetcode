//! 找到最大非递减数组的长度


use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

/// 不能贪心，DP做法，O(n^2)  （过不了）
pub fn find_maximum_length(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut dp = vec![0; len + 1]; // dp[i] 表示前 i 个数的最大非递减数组长度
    let mut last = vec![0; len + 1];  // last[i] 表示 dp[i] 情况下，最后一个数的值
    // 我们要让 dp[i] 最大的情况下， last[i] 最小
    for i in 1..=len {
        dp[i] = dp[i - 1];
        last[i] = last[i - 1] + nums[i - 1] as i64;
        for j in 0..i {
            if presum[i] - presum[j] >= last[j] {  // j+1:i 的和 >= last[j] 满足题目条件
                if (dp[j] + 1 > dp[i]) || (dp[j] + 1 == dp[i] && (presum[i] - presum[j] < last[i])) {
                    dp[i] = dp[j] + 1;
                    last[i] = presum[i] - presum[j];
                }
            }
        }
    }
    dp[len]
}

/// 对于DP做法，考虑优化。 主要优化思路是优化 dp[i] 从 dp[0..i]转移的过程
/// 对于在 0..i 范围的j ，需要筛选 presum[i] - presum[j] >= last[j]
/// 即 presum[i] >= presum[j] + last[j]。 用堆去筛选
pub fn find_maximum_length2(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut heap: BinaryHeap<(i64, i32, i64)> = BinaryHeap::new();
    let mut presum = 0;
    let mut dp = 1;
    let mut last = 0;
    for num in nums {
        presum += num as i64;
        last += num as i64;
        while !heap.is_empty() && -heap.peek().unwrap().0 <= presum {
            let (_, dp1, presum1) = heap.pop().unwrap();
            if dp1 + 1 > dp || (dp1 + 1 == dp && (presum - presum1) < last) {
                dp = dp1 + 1;
                last = presum - presum1;
            }
        }
        heap.push((-presum - last, dp, presum));
    }
    dp
}

/// 单调栈二分查找
/// 对于 j 和 k，如果 j < k 时， presum[i] >= presum[j] + last[j] >= presum[k] + last[k]，我们可以直接忽略 j。
pub fn find_maximum_length3(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut dp = vec![0; len + 1];
    let mut last = vec![0; len + 1];
    let mut st = vec![(0, 0)]; // (presum[i] + last[i], i)  单调增
    for i in 1..=len {
        let sti = st.binary_search_by(|x| x.0.cmp(&presum[i]).then(Ordering::Less)).unwrap_err();  // 找 > presum[i] 的第一个数
        let j = st[sti - 1].1; //  >= presum[i] 的最后一个数
        dp[i] = dp[j] + 1;
        last[i] = presum[i] - presum[j];
        while !st.is_empty() && st[st.len() - 1].0 >= presum[i] + last[i] {
            st.pop();
        }
        st.push((presum[i] + last[i], i));
    }
    dp[len]
}

/// 对单调栈的优化：单调队列，时间复杂度：O(n)
pub fn find_maximum_length4(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut dp = vec![0; len + 1];
    let mut last = vec![0; len + 1];
    let mut q = VecDeque::with_capacity(len + 1);
    q.push_back(0);
    for i in 1..=len {
        // 去掉队首无用的数据
        while q.len() > 1 && presum[q[1]] + last[q[1]] <= presum[i] {  // VecDeque 队头可以用下标拿
            q.pop_front().unwrap();
        }
        dp[i] = dp[q[0]] + 1;
        last[i] = presum[i] - presum[q[0]];
        // 去掉队尾无用的数据
        while !q.is_empty() && presum[*q.back().unwrap()] + last[*q.back().unwrap()] >= presum[i] + last[i] {  // 队尾要用back取
            q.pop_back();
        }
        q.push_back(i);
    }
    dp[len]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![147, 633, 535, 183, 886]), 4);
        assert_eq!(func(vec![336, 78, 256, 976, 976, 764, 370, 46]), 4);
        assert_eq!(func(vec![417, 241, 895, 308, 259, 562]), 3);
        assert_eq!(func(vec![4, 3, 2, 6]), 3);
        assert_eq!(func(vec![5, 4, 3, 2, 1]), 2);
        assert_eq!(func(vec![272, 482, 115, 925, 983]), 4);
        assert_eq!(func(vec![5, 2, 2]), 1);
        assert_eq!(func(vec![1, 2, 3, 4]), 4);
    }
    test(find_maximum_length);
    test(find_maximum_length2);
    test(find_maximum_length3);
    test(find_maximum_length4);
}
