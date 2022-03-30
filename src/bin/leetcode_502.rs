//! IPO

use std::collections::BinaryHeap;

pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    let len = profits.len();
    let mut cp: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
    cp.sort_unstable();
    let mut i = 0;
    for _ in 0..k {
        while i < len && cp[i].0 <= w {
            heap.push(cp[i].1);
            i += 1;
        }
        if heap.is_empty() { break; }
        let profit = heap.pop().unwrap();
        w += profit;
    }
    w
}

fn main() {
    assert_eq!(find_maximized_capital(11, 11, vec![1, 2, 3], vec![11, 12, 13]), 17);
    assert_eq!(find_maximized_capital(1, 2, vec![1, 2, 3], vec![1, 1, 2]), 5);
    assert_eq!(find_maximized_capital(1, 0, vec![1, 2, 3], vec![1, 1, 2]), 0);
    assert_eq!(find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]), 4);
    assert_eq!(find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]), 6);
}
