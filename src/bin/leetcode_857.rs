//! 雇佣 K 名工人的最低成本

use std::collections::BinaryHeap;

/// 按价性比排序（选中的按最大的价性比计算价格），用堆计算和最小的质量总数
pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let mut workers: Vec<(f64, i32)> = quality.into_iter().zip(wage).map(|(q, w)| {
        (w as f64 / q as f64, q)
    }).collect();
    workers.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut result = f64::MAX;
    let mut qsum = 0;
    let mut heap = BinaryHeap::new();
    let k = k as usize;
    for worker in workers {
        qsum += worker.1;
        heap.push(worker.1);
        if heap.len() > k {
            qsum -= heap.pop().unwrap();
        }
        if heap.len() == k {
            result = result.min(qsum as f64 * worker.0)
        }
    }
    result
}

fn main() {
    assert_eq!(mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2), 105.00000);
    assert_eq!(mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3), 30.66667);
}
