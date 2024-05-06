//! 找出与数组相加的整数 I

pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    nums2.into_iter().min().unwrap() - nums1.into_iter().min().unwrap()
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 6, 4], vec![9, 7, 5]), 3);
        assert_eq!(func(vec![10], vec![5]), -5);
        assert_eq!(func(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), 0);
    }
    test(added_integer);
}
