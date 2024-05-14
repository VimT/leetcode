//! 找出强数对的最大异或值 I

pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            if (nums[i] - nums[j]).abs() <= nums[i].min(nums[j]) {
                result = result.max(nums[i] ^ nums[j]);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(func(vec![10, 100]), 0);
        assert_eq!(func(vec![5, 6, 25, 30]), 7);
    }
    test(maximum_strong_pair_xor);
}
