//! 二分查找

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 { return 0; }
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] == target {
            return mid as i32;
        } else {
            right = mid;
        }
    }
    left as i32
}

fn main() {
    assert_eq!(search_insert(vec![1, 3, 5, 7, 9], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 7, 9], 0), 0);
    assert_eq!(search_insert(vec![1, 3, 5, 7, 9], 11), 5);
}
