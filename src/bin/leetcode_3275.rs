//! 第 K 近障碍物查询

use std::collections::BinaryHeap;

pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    queries.into_iter().map(|q| {
        let distance = q[0].abs() + q[1].abs();
        heap.push(distance);
        if heap.len() > k as usize { heap.pop(); }
        if heap.len() < k as usize { -1 } else { *heap.peek().unwrap() }
    }).collect()
}

fn main() {
    fn test(func: fn(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![-3, 0]], 2), vec![-1, 7, 5, 3]);
        assert_eq!(func(vec![vec![5, 5], vec![4, 4], vec![3, 3]], 1), vec![10, 8, 6]);
    }
    test(results_array);
}
