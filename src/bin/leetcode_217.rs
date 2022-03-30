//! 存在重复元素

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
    return len != set.len();
}

fn main() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}
