//! 二分查找

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (left + right) >> 1;
        if nums[mid] > target {
            if mid == 0 { return -1; }
            right = mid - 1;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            return mid as i32;
        }
    }
    -1
}

fn main() {
    assert_eq!(search(vec![5], -5), -1);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
