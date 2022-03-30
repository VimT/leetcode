//! 搜索旋转排序数组


pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 { return -1; }
    let mut left = 0;
    let mut right = nums.len() - 1;
    let left_min = nums[left];
    let right_max = nums[right];
    while left < right {
        let mid_index = left + (right - left) / 2;
        let mid = nums[mid_index];
        if mid == target { return mid_index as i32; }
        if target <= right_max {
            // target 在右半段
            if mid > right_max {
                left = mid_index + 1;
            } else {
                if mid > target { right = mid_index; } else { left = mid_index + 1; }
            }
        } else {
            if mid < left_min {
                right = mid_index;
            } else {
                if mid > target { right = mid_index; } else { left = mid_index + 1; }
            }
        }
    }
    return if nums[left] == target { left as i32 } else { -1 };
}

fn main() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(search(vec![1], 0), -1);
}
