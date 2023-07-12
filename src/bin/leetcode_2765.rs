//! 最长交替子序列

pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = -1;
    for i in 0..len - 1 {
        if nums[i + 1] - nums[i] == 1 {
            let mut j = i + 1;
            let mut s = 1;
            while j < len && nums[j] - nums[j - 1] == s {
                j += 1;
                s = -s;
                result = result.max((j - i) as i32);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![21, 9, 5]), -1);
        assert_eq!(func(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(func(vec![4, 5, 6]), 2);
    }
    test(alternating_subarray);
}
