//! 找出数组的第 K 大和

use std::collections::BinaryHeap;

pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
    let mut sum = 0;
    for num in &mut nums {
        if *num >= 0 { sum += *num as i64 } else { *num = -*num; }
    }
    nums.sort_unstable();
    let mut heap = BinaryHeap::new();
    heap.push((sum, 0));
    for _ in 1..k {
        let (sum, i) = heap.pop().unwrap();
        if i < nums.len() {
            // 保留nums[i-1]
            heap.push((sum - nums[i] as i64, i + 1));
            if i > 0 {
                // 不保留num[i-1]
                heap.push((sum - nums[i] as i64 + nums[i - 1] as i64, i + 1));
            }
        }
    }
    heap.pop().unwrap().0
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![1, -2, 3, 4, -10, 12], 16), 10);
        assert_eq!(func(vec![2, 4, -2], 5), 2);
    }
    test(k_sum);
}
