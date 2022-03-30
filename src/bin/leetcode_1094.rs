//! 拼车

use std::collections::BinaryHeap;

pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut heap = BinaryHeap::new();
    for trip in trips {
        heap.push((-trip[1], -trip[0]));
        heap.push((-trip[2], trip[0]));
    }
    let mut cur = 0;
    while !heap.is_empty() {
        let (_, add) = heap.pop().unwrap();
        cur -= add;
        if cur > capacity {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(trips: Vec<Vec<i32>>, capacity: i32) -> bool) {
        assert_eq!(func(vec![vec![2, 1, 5], vec![3, 5, 7]], 3), true);
        assert_eq!(func(vec![vec![2, 1, 5], vec![3, 3, 7]], 4), false);
        assert_eq!(func(vec![vec![2, 1, 5], vec![3, 3, 7]], 5), true);
    }
    test(car_pooling);
}
