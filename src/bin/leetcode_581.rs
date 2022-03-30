//! 最短无序连续子数组

pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut right_min = i32::MAX;
    let mut left_max = i32::MIN;
    let mut left = len;
    let mut right = 0;
    for i in (0..len).rev() {
        right_min = right_min.min(nums[i]);
        if right_min < nums[i] {
            left = i;
        }
    }
    if left == len { return 0; }
    for i in 0..len {
        left_max = left_max.max(nums[i]);
        if left_max > nums[i] {
            right = i;
        }
    }

    (right - left + 1) as i32
}

fn main() {
    assert_eq!(find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
    assert_eq!(find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    assert_eq!(find_unsorted_subarray(vec![1]), 0);
}
