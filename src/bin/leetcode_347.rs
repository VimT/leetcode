//! 前 K 个高频元素

use std::collections::{BinaryHeap, HashMap};

/// top k 高频出现的数字。。先counter，再小堆。。BinaryHeap没有key元素，只能用KV实现一个
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counter = HashMap::new();
    let mut ans: Vec<i32> = vec![];
    for i in nums {
        counter.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut heap = BinaryHeap::new();
    for (num, count) in counter {
        heap.push((-count, num));
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    for _ in 0..k {
        ans.push(heap.pop().unwrap().1);
    }
    ans.reverse();
    ans
}

fn main() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
}
