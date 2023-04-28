//! 不同整数的最少数目

use std::collections::{BinaryHeap, HashMap};

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for num in arr {
        *cnt.entry(num).or_default() -= 1;
    }
    let mut cnt: BinaryHeap<i32> = cnt.values().copied().collect();
    while !cnt.is_empty() {
        let num_cnt = cnt.pop().unwrap();
        k += num_cnt;
        if k <= 0 {
            return cnt.len() as i32 + (k < 0) as i32;
        }
    }
    0
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![5, 5, 4], 1), 1);
        assert_eq!(func(vec![4, 3, 1, 1, 3, 3, 2], 3), 2);
    }
    test(find_least_num_of_unique_ints);
}
