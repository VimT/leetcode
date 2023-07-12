//! 重新放置石块


use std::collections::HashSet;

pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
    let mut cnt: HashSet<i32> = nums.into_iter().collect();
    for (f, t) in move_from.into_iter().zip(move_to) {
        if cnt.remove(&f) {
            cnt.insert(t);
        }
    }
    let mut pos: Vec<i32> = cnt.into_iter().collect();
    pos.sort_unstable();
    pos
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5]), vec![5, 6, 8, 9]);
        assert_eq!(func(vec![1, 1, 3, 3], vec![1, 3], vec![2, 2]), vec![2]);
    }
    test(relocate_marbles);
}
