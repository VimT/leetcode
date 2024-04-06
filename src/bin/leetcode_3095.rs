//! 或值至少 K 的最短子数组 I

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut result = i32::MAX;
    for i in 0..len {
        let mut or = 0;
        for j in i..len {
            or |= nums[j];
            if or >= k {
                result = result.min((j - i + 1) as i32);
                break;
            }
        }
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3], 2), 1);
        assert_eq!(func(vec![2, 1, 8], 10), 3);
        assert_eq!(func(vec![1, 2], 0), 1);
    }
    test(minimum_subarray_length);
}
