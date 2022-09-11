//! 最长优雅子数组

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;
    let mut cur = 0;
    while j < len {
        while j < len && nums[j] & cur == 0 {
            cur |= nums[j];
            j += 1;
        }
        result = result.max(j - i);
        cur ^= nums[i];
        i += 1;
    }
    result as i32
}

fn main() {
    assert_eq!(longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    assert_eq!(longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
}
