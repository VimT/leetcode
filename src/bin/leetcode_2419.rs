//! 按位与最大的最长子数组

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    let mut result = 1;
    let mut cur = 0;
    for num in nums {
        if num == max {
            cur += 1;
            result = result.max(cur);
        } else {
            cur = 0;
        }
    }
    result
}

fn main() {
    assert_eq!(longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
    assert_eq!(longest_subarray(vec![1, 2, 3, 4]), 1);
}
