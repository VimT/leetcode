//! 在排序数组中查找元素的第一个和最后一个位置

/// 注意二分查找的 比较，决定了结果是最左边还是最右边。
/// 注意right=len
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    if len == 0 { return vec![-1, -1]; }
    let mut left = 0;
    let mut right = len;
    let mut tl = 0;
    let mut tr = 0;

    while left < right {
        let mid = left + (right - left) / 2;
        if target <= nums[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left == len || nums[left] != target { return vec![-1, -1]; }
    tl = left;
    left = 0;
    right = len;

    while left < right {
        let mid = left + (right - left) / 2;
        if target < nums[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    tr = left - 1;
    vec![tl as i32, tr as i32]
}

fn main() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    assert_eq!(search_range(vec![], 0), vec![-1, -1]);
}
