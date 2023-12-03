//! 有序三元组中的最大值 I

pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            for k in i + 2..len {
                result = result.max((nums[i] - nums[j]) as i64 * nums[k] as i64);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(func(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(func(vec![1, 2, 3]), 0);
    }
    test(maximum_triplet_value);
}
