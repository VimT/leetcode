//! 搜索旋转排序数组 II

/// 注意这里的二分写法
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 0 { return false; }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[left] == nums[mid] {
            left += 1;
            continue;
        }
        if nums[left] < nums[mid] {
            // mid在前半部分
            if nums[mid] > target && nums[left] <= target {
                // nums[left] <= target < nums[mid]
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // mid在后半部分
            if nums[mid] < target && nums[right] >= target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    return false;
}

fn main() {
    assert_eq!(search(vec![1, 3, 5], 1), true);
    assert_eq!(search(vec![1, 3, 1, 1], 3), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 2), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    assert_eq!(search(vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1], 1), true);
    assert_eq!(search(vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1], 2), true);
    assert_eq!(search(vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1], 3), false);
    assert_eq!(search(vec![3, 3, 3, 3, 3, 3, 3], 3), true);
    assert_eq!(search(vec![1], 2), false);
}

