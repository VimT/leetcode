//! 统计和小于目标的下标对数目

pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut result = 0;
    let len = nums.len();
    for i in 0..len {
        for j in i + 1..len {
            if nums[i] + nums[j] < target {
                result += 1;
            }
        }
    }
    result
}

/// O(nlogn)
pub fn count_pairs2(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut result = 0;
    let mut left = 0;
    let mut right = nums.len();
    while left + 1 < right {
        if nums[left] + nums[right - 1] < target {
            result += right - left - 1;
            left += 1;
        } else {
            right -= 1;
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(func(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
    test(count_pairs);
    test(count_pairs2);
}
