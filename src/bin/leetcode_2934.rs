//! 最大化数组末位元素的最少操作次数

pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    fn calc(nums1: &Vec<i32>, nums2: &Vec<i32>, a: i32, b: i32) -> Option<i32> {
        let len = nums1.len();
        let mut result = 0;
        for i in 0..len - 1 {
            if !(nums1[i] <= a && nums2[i] <= b) {
                if nums2[i] <= a && nums1[i] <= b {
                    result += 1;
                } else {
                    return None;
                }
            }
        }
        Some(result)
    }
    let a = nums1.last().copied().unwrap();
    let b = nums2.last().copied().unwrap();
    let r1 = calc(&nums1, &nums2, a, b);
    let r2 = calc(&nums1, &nums2, b, a).map(|x| x + 1);
    match (r1, r2) {
        (Some(x), Some(y)) => x.min(y),
        (Some(x), None) => x,
        (None, Some(y)) => y,
        (None, None) => -1,
    }
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 7], vec![4, 5, 3]), 1);
        assert_eq!(func(vec![2, 3, 4, 5, 9], vec![8, 8, 4, 4, 4]), 2);
        assert_eq!(func(vec![1, 5, 4], vec![2, 5, 3]), -1);
    }
    test(min_operations);
}
