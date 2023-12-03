//! 有序三元组中的最大值 II

/// 枚举 j
pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut right_mx = vec![0; len];
    let mut mx = 0;
    for i in (0..len).rev() {
        right_mx[i] = mx;
        mx = mx.max(nums[i]);
    }
    mx = 0;
    let mut result = 0;
    for (i, num) in nums.into_iter().enumerate() {
        result = result.max((mx - num) as i64 * right_mx[i] as i64);
        mx = mx.max(num);
    }
    result
}

/// 枚举 k
pub fn maximum_triplet_value2(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut mx = 0;
    let mut mx_diff = 0;
    for num in nums {
        result = result.max(mx_diff as i64 * num as i64);
        mx_diff = mx_diff.max(mx - num);
        mx = mx.max(num);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1000000, 1, 1000000]), 999999000000);
        assert_eq!(func(vec![15, 12, 2, 14, 15, 18, 15, 20, 14, 5, 14, 14, 11, 13, 7]), 260);
        assert_eq!(func(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(func(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(func(vec![1, 2, 3]), 0);
    }
    test(maximum_triplet_value);
    test(maximum_triplet_value2);
}
