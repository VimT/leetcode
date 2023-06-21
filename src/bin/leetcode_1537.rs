//! 最大得分

use std::cmp::Ordering;

pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mut prei = 0;
    let mut prej = 0;
    let mut result = 0;
    while i < len1 && j < len2 {
        match nums1[i].cmp(&nums2[j]) {
            Ordering::Less => {
                prei += nums1[i] as i64;
                i += 1;
            }
            Ordering::Equal => {
                result += prei.max(prej) + nums1[i] as i64;
                prei = 0;
                prej = 0;
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                prej += nums2[j] as i64;
                j += 1;
            }
        }
    }
    for k in i..len1 { prei += nums1[k] as i64; }
    for k in j..len2 { prej += nums2[k] as i64; }
    result += prei.max(prej);
    (result % (1e9 as i64 + 7)) as i32
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]), 30);
        assert_eq!(func(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
        assert_eq!(func(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 40);
        assert_eq!(func(vec![1, 4, 5, 8, 9, 11, 19], vec![2, 3, 4, 11, 12]), 61);
    }
    test(max_sum);
}
