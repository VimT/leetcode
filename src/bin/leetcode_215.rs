//! 数组中的第K个最大元素

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use leetcode::algorithm;

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let k = k as usize - 1;
    while start < end {
        let i = algorithm::partition_reverse(&mut nums, start, end);
        if i == k {
            return nums[i];
        } else if i > k {
            end = i - 1;
        } else {
            start = i + 1;
        }
    }
    nums[start]
}

/// 堆排序法，只需要建立k大小的小顶堆
pub fn find_kth_largest_heap(mut nums: Vec<i32>, k: i32) -> i32 {
    // 堆化
    let k = k as usize;
    for i in (0..k / 2).rev() {
        algorithm::min_heapify(&mut nums, i, k - 1);
    }
    for i in k..nums.len() {
        if nums[i] > nums[0] {
            nums.swap(i, 0);
            algorithm::min_heapify(&mut nums, 0, k - 1);
        }
    }
    return nums[0];
}

pub fn find_kth_largest_binheap(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for i in nums {
        heap.push(Reverse(i));
        if heap.len() > k {
            heap.pop();
        }
    }
    return heap.pop().unwrap().0;
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(func(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
    test(find_kth_largest);
    test(find_kth_largest_binheap);
    test(find_kth_largest_heap);
}
