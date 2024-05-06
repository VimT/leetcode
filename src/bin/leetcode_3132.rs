//! 找出与数组相加的整数 II

pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort_unstable();
    nums2.sort_unstable();
    // 枚举保留 nums1[2] 或者 nums1[1] 或者 nums1[0]
    for i in (1..=2).rev() {
        let diff = nums2[0] - nums1[i];
        // 在 (nums1[i] + diff) 中找子序列 nums2
        let mut j = 0;
        for &v in &nums1[i..] {
            if j < nums2.len() && nums2[j] == v + diff {
                j += 1;
                // nums2 是 nums1[i] + diff 的子序列
                if j == nums2.len() {
                    return diff;
                }
            }
        }
    }
    nums2[0] - nums1[0]
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 20, 16, 12, 8], vec![14, 18, 10]), -2);
        assert_eq!(func(vec![3, 5, 5, 3], vec![7, 7]), 2);
    }
    test(minimum_added_integer);
}
