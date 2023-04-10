//! 从两个数字数组里生成最小数字

pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut result = 99;
    for n1 in nums1 {
        for &n2 in &nums2 {
            if n1 == n2 {
                result = result.min(n1);
            } else {
                result = result.min(if n1 > n2 { n2 * 10 + n1 } else { n1 * 10 + n2 });
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 1, 3], vec![5, 7]), 15);
        assert_eq!(func(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
    }
    test(min_number);
}
