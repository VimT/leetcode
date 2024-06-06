//! 优质数对的总数 I

pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let n = nums1.len();
    let m = nums2.len();
    let mut result = 0;
    for i in 0..n {
        for j in 0..m {
            if nums1[i] % (nums2[j] * k) == 0 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
        assert_eq!(func(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
    }
    test(number_of_pairs);
}
