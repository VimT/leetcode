//! 最长奇偶子数组

pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let len = nums.len();
    let mut i = 0;
    let mut result = 0;
    while i < len {
        if nums[i] % 2 != 0 || nums[i] > threshold {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < len && nums[j] <= threshold && nums[j] % 2 != nums[j - 1] % 2 {
            j += 1;
        }
        result = result.max(j - i);
        i = j;
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, threshold: i32) -> i32) {
        assert_eq!(func(vec![1], 1), 0);
        assert_eq!(func(vec![2], 1), 0);
        assert_eq!(func(vec![2], 2), 1);
        assert_eq!(func(vec![3, 2, 5, 4], 5), 3);
        assert_eq!(func(vec![1, 2], 2), 1);
        assert_eq!(func(vec![2, 3, 4, 5], 4), 3);
    }
    test(longest_alternating_subarray);
}
