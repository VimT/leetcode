//! 标记所有下标的最早秒数 II

use std::collections::BinaryHeap;

/// 反悔贪心
pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = change_indices.len();
    let mut first_t = vec![m; n];  // i 在 change_indices 中第一次出现的时间
    for t in (0..m).rev() {
        first_t[(change_indices[t] - 1) as usize] = t;
    }
    let total = n as i64 + nums.iter().map(|&x| x as i64).sum::<i64>();  // 慢速标记需要的时间

    let check = |mx: usize| -> bool {
        let mut cnt = 0;  // 可用天数
        let mut slow = total;
        let mut heap: BinaryHeap<i64> = BinaryHeap::new();
        for t in (0..mx).rev() {
            let i = change_indices[t] as usize - 1;
            let v = nums[i] as i64;
            if v <= 1 || t != first_t[i] {
                cnt += 1;  // 留给左边处理
                continue;
            }
            if cnt == 0 {
                if heap.is_empty() || v <= -*heap.peek().unwrap() {
                    cnt += 1; // 留给左边处理
                    continue;
                }
                // 反悔一个比 nums[i] 小的数
                slow += -heap.pop().unwrap() + 1;
                cnt += 2; // 反悔，一天到0，一天标记
            }
            // 快速到0，消耗一天标记
            slow -= v + 1;
            cnt -= 1;
            heap.push(-v);
        }
        cnt >= slow  // 剩余天数搞定慢速复习+考试
    };

    let mut left = n;
    let mut right = m + 1;
    while left < right {
        let mid = (left + right) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left > m { -1 } else { left as i32 }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, change_indices: Vec<i32>) -> i32) {
        assert_eq!(func(vec![100, 2, 2, 2], vec![1, 1, 1, 1, 1, 1, 2, 3, 4]), -1);
        assert_eq!(func(vec![7, 1, 0, 10], vec![1, 1, 1, 3, 4, 2, 2, 2, 4, 4, 2, 3]), 7);
        assert_eq!(func(vec![5, 1, 3, 2, 2, 5], vec![3, 2, 2, 3, 1, 1, 3, 4, 2, 3, 4, 2, 5, 6, 5, 3, 6, 5, 3]), 15);
        assert_eq!(func(vec![0, 0, 3, 0], vec![2, 2, 2, 2, 2, 2, 3, 1]), 7);
        assert_eq!(func(vec![0, 2], vec![1, 1, 2, 2, 1]), 4);
        assert_eq!(func(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3]), 6);
        assert_eq!(func(vec![0, 0, 1, 2], vec![1, 2, 1, 2, 1, 2, 1, 2]), 7);
        assert_eq!(func(vec![1, 2, 3], vec![1, 2, 3]), -1);
    }
    test(earliest_second_to_mark_indices);
}
