//! 不同的平均值数目

use std::collections::HashSet;

pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    (0..len / 2).map(|x| nums[x] + nums[len - 1 - x]).collect::<HashSet<i32>>().len() as i32
}

fn main() {
    assert_eq!(distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
    assert_eq!(distinct_averages(vec![1, 100]), 1);
}
