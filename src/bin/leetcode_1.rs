//! 两数之和

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();
    for i in 0..nums.len() {
        m.insert(nums[i], i);
    }
    for i in 0..nums.len() {
        if let Some(o) = m.get(&(target - nums[i])) {
            if *o != i {
                return vec![i as i32, *o as i32];
            }
        }
    }
    panic!("error");
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
