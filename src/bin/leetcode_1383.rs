//! 最大的团队表现值

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut se: Vec<(i32, i32)> = efficiency.into_iter().zip(speed).collect();
    se.sort_unstable();
    let mut heap = BinaryHeap::with_capacity(k + 1);
    let mut cur_sum = 0;
    let mut result = 0;
    for (eff, speed) in se.into_iter().rev() {
        heap.push(Reverse(speed as u64));
        cur_sum += speed as u64;
        if heap.len() > k {
            let Reverse(pop) = heap.pop().unwrap();
            cur_sum -= pop;
        }
        result = result.max(eff as u64 * cur_sum);
    }
    (result % (1e9 as u64 + 7)) as i32
}

fn main() {
    fn test(func: fn(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(3, vec![2, 8, 2], vec![2, 7, 1], 2), 56);
        assert_eq!(func(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2), 60);
        assert_eq!(func(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3), 68);
        assert_eq!(func(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4), 72);
    }
    test(max_performance);
}
