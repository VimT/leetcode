//! 所有数对的异或和

pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut result = 0;
    let len1 = nums1.len();
    if nums2.len() & 1 == 1 {
        for num in nums1 {
            result ^= num;
        }
    }
    if len1 & 1 == 1 {
        for num in nums2 {
            result ^= num;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
        assert_eq!(func(vec![1, 2], vec![3, 4]), 0);
    }
    test(xor_all_nums);
}
