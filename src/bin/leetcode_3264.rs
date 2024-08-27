//! K 次乘运算后的最终数组 I

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut heap: BinaryHeap<_> = nums.iter().copied().zip(0..).map(Reverse).collect();
    for _ in 0..k {
        let Reverse((num, idx)) = heap.pop().unwrap();
        heap.push(Reverse((num * multiplier, idx)));
    }
    for Reverse((num, idx)) in heap {
        nums[idx] = num;
    }
    nums
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32>) {
        assert_eq!(func(vec![2, 1, 3, 5, 6], 5, 2), vec![8, 4, 6, 5, 6]);
        assert_eq!(func(vec![1, 2], 3, 4), vec![16, 8]);
    }
    test(get_final_state);
}
