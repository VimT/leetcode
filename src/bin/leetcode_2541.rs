//! 使数组中所有元素相等的最小操作数 II


pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let sum1: i64 = nums1.iter().map(|x| *x as i64).sum();
    let sum2: i64 = nums2.iter().map(|x| *x as i64).sum();
    if sum1 != sum2 { return -1; }
    let len = nums1.len();
    let mut result = 0;
    for i in 0..len {
        if nums1[i] != nums2[i] {
            let diff = (nums1[i] - nums2[i]).abs();
            if k == 0 || diff % k != 0 {
                return -1;
            }
            result += (diff / k) as i64;
        }
    }
    result / 2
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3), 2);
        assert_eq!(func(vec![3, 8, 5, 2], vec![2, 4, 1, 6], 1), -1);
    }
    test(min_operations);
}
