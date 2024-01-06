//! 找到两个数组中的公共元素

pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut cnt1 = vec![0; 101];
    let mut cnt2 = vec![0; 101];
    for num in nums1 {
        cnt1[num as usize] += 1;
    }
    for num in nums2 {
        cnt2[num as usize] += 1;
    }
    let mut result1 = 0;
    let mut result2 = 0;
    for i in 0..=100 {
        if cnt1[i] > 0 && cnt2[i] > 0 {
            result1 += cnt1[i];
            result2 += cnt2[i];
        }
    }
    vec![result1, result2]
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]), vec![3, 4]);
        assert_eq!(func(vec![3, 4, 2, 3], vec![1, 5]), vec![0, 0]);
    }
    test(find_intersection_values);
}
