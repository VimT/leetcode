//! 移除后集合的最多元素数

use std::collections::HashSet;

pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len = nums1.len();
    let set1 = nums1.into_iter().collect::<HashSet<i32>>();
    let set2 = nums2.into_iter().collect::<HashSet<i32>>();
    let mut same = set1.intersection(&set2).count();
    let mut only1 = set1.len() - same;
    let mut only2 = set2.len() - same;
    while same > 0 && (only1 < len / 2 || only2 < len / 2) {
        same -= 1;
        if only1 < len / 2 {
            only1 += 1;
        } else if only2 < len / 2 {
            only2 += 1;
        }
    }
    (only1.min(len / 2) + only2.min(len / 2)) as i32
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]), 5);
        assert_eq!(func(vec![1, 2, 1, 2], vec![1, 1, 1, 1]), 2);
        assert_eq!(func(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]), 6);
    }
    test(maximum_set_size);
}
