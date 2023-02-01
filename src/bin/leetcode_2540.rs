//! 最小公共值

use std::cmp::Ordering;

pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            Ordering::Less => {
                i += 1;
            }
            Ordering::Equal => {
                return nums1[i];
            }
            Ordering::Greater => {
                j += 1;
            }
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3], vec![2, 4]), 2);
        assert_eq!(func(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
    test(get_common);
}
