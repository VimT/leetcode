//! 有序数组中的单一元素

pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let len = nums.len();
    let mut right = len;
    while left < right {
        let mid = (left + right) / 2;
        let is_match = if mid & 1 == 0 { mid + 1 < len && nums[mid] == nums[mid + 1] } else { nums[mid] == nums[mid - 1] };
        if is_match {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    nums[left]
}

fn main() {
    assert_eq!(single_non_duplicate(vec![0, 0, 1]), 1);
    assert_eq!(single_non_duplicate(vec![0]), 0);
    assert_eq!(single_non_duplicate(vec![1, 2, 2, 3, 3, 4, 4, 8, 8]), 1);
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 2, 3, 3, 4, 4, 8]), 8);
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
