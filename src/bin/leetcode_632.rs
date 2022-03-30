//! 最小区间


use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// 堆
/// 该问题可以转化为，从 k 个列表中各取一个数，使得这 k 个数中的最大值与最小值的差最小。
/// k个列表取一个数，组最小堆，则堆的最小值，和堆的最大值，就是此区间。
/// 遍历整个数据，依次入堆，出堆。更新区间，则得到最小区间
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heap = BinaryHeap::with_capacity(nums.len());
    let mut current_max = i32::min_value();
    let mut current_idx = vec![0; nums.len()];
    for i in 0..nums.len() {
        heap.push((Reverse(nums[i][0]), i));
        current_max = current_max.max(nums[i][0]);
    }
    let mut ans0 = 0;
    let mut ans1 = i32::max_value();
    loop {
        let (Reverse(min), idx) = heap.pop().unwrap();
        if (current_max - min) < (ans1 - ans0) {
            ans0 = min;
            ans1 = current_max;
        }
        current_idx[idx] += 1;
        if current_idx[idx] == nums[idx].len() {
            break;
        }
        let new_value = nums[idx][current_idx[idx]];
        heap.push((Reverse(new_value), idx));
        current_max = current_max.max(new_value);
    }
    vec![ans0, ans1]
}


/// 滑动窗口，类似 76 题
pub fn smallest_range_window(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums.len();
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in 0..len {
        for x in &nums[i] {
            map.entry(*x).or_default().push(i);
            min = min.min(*x);
            max = max.max(*x);
        }
    }
    let mut left = min;
    let mut right = min - 1;
    let mut ans0 = 0;
    let mut ans1 = i32::MAX;

    let mut num_size = vec![0; len];
    let mut success = 0;
    while right < max {
        right += 1;
        if let Some(idxs) = map.get(&right) {
            for i in idxs {
                if num_size[*i] == 0 {
                    success += 1;
                }
                num_size[*i] += 1;
            }
        }

        while success == len {
            if (right - left) < (ans1 - ans0) {
                ans0 = left;
                ans1 = right;
            }
            if let Some(idxs) = map.get(&left) {
                for i in idxs {
                    num_size[*i] -= 1;
                    if num_size[*i] == 0 {
                        success -= 1;
                    }
                }
            }
            left += 1;
        }
    }
    vec![ans0, ans1]
}

fn main() {
    fn test(func: fn(nums: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![4, 10, 15, 24, 26], vec![0, 9, 12, 20], vec![5, 18, 22, 30]]), vec![20, 24]);
        assert_eq!(func(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]), vec![1, 1]);
    }
    test(smallest_range);
    test(smallest_range_window);
}
