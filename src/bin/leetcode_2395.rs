//! 和相等的子数组

use std::collections::HashSet;

pub fn find_subarrays(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for win in nums.windows(2) {
        let sum = win[0] + win[1];
        if set.contains(&sum) {
            return true;
        }
        set.insert(sum);
    }
    false
}

fn main() {
    assert_eq!(find_subarrays(vec![4, 2, 4]), true);
    assert_eq!(find_subarrays(vec![1, 2, 3, 4, 5]), false);
    assert_eq!(find_subarrays(vec![0, 0, 0]), true);
}
