//! 从数量最多的堆取走礼物

use std::collections::BinaryHeap;

pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<i32> = gifts.into_iter().collect();
    for _ in 0..k {
        if heap.peek().cloned().unwrap() == 1 { break; }
        let gift = heap.pop().unwrap();
        heap.push((gift as f64).sqrt() as i32);
    }
    heap.into_iter().map(|x| x as i64).sum::<i64>()
}

fn main() {
    fn test(func: fn(gifts: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![25, 64, 9, 4, 100], 4), 29);
        assert_eq!(func(vec![1, 1, 1, 1], 4), 4);
    }
    test(pick_gifts);
}
