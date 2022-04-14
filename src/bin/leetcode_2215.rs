//! 找出两数组的不同

use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let s1: HashSet<i32> = nums1.into_iter().collect();
    let s2: HashSet<i32> = nums2.into_iter().collect();
    vec![s1.difference(&s2).cloned().collect(), s2.difference(&s1).cloned().collect()]
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>>) {
        let wrap = |mut ans: Vec<Vec<i32>>| {
            ans[0].sort_unstable();
            ans[1].sort_unstable();
            ans
        };
        assert_eq!(wrap(func(vec![1, 2, 3], vec![2, 4, 6])), vec![vec![1, 3], vec![4, 6]]);
        assert_eq!(wrap(func(vec![1, 2, 3, 3], vec![1, 1, 2, 2])), vec![vec![3], vec![]]);
    }
    test(find_difference);
}
