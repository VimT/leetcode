//! 执行操作标记数组中的元素

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let len = nums.len();
    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for i in 0..len {
        heap.push(Reverse((nums[i], i)));
        sum += nums[i] as i64;
    }
    let mut mark = vec![false; len];
    queries.into_iter().map(|x| {
        let idx = x[0] as usize;
        let k = x[1];
        if !mark[idx] {
            sum -= nums[idx] as i64;
            mark[idx] = true;
        }
        let mut k = k;
        while k > 0 {
            if heap.is_empty() { break; }
            let Reverse((num, i)) = heap.pop().unwrap();
            if !mark[i] {
                sum -= num as i64;
                mark[i] = true;
                k -= 1;
            }
        }
        sum
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64>) {
        assert_eq!(func(vec![1, 2, 2, 1, 2, 3, 1], vec![vec![1, 2], vec![3, 3], vec![4, 2]]), vec![8, 3, 0]);
        assert_eq!(func(vec![1, 4, 2, 3], vec![vec![0, 1]]), vec![7]);
    }
    test(unmarked_sum_array);
}
