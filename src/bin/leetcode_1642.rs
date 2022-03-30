//! 可以到达的最远建筑

use std::collections::BinaryHeap;

pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let len = heights.len();
    for i in 1..len {
        if heights[i] > heights[i - 1] {
            let diff = heights[i] - heights[i - 1];
            heap.push(diff);
            bricks -= diff;
            if bricks < 0 {
                if ladders > 0 {
                    ladders -= 1;
                    bricks += heap.pop().unwrap();
                } else {
                    return (i - 1) as i32;
                }
            }
        }
    }
    (len - 1) as i32
}

fn main() {
    assert_eq!(furthest_building(vec![1, 2, 3, 4], 5, 0), 3);
    assert_eq!(furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1), 4);
    assert_eq!(furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2), 7);
    assert_eq!(furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
}

