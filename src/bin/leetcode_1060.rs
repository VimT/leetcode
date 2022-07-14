//! 有序数组中的缺失元素

pub fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut left = 0;
    let mut right = len;
    while left < right {
        let mid = (left + right) / 2;
        let leak = nums[mid] - nums[0] - mid as i32;
        if leak >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    nums[left - 1] + k - (nums[left - 1] - nums[0] - left as i32 + 1)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![4, 7, 9, 10], 1), 5);
        assert_eq!(func(vec![4, 7, 9, 10], 3), 8);
        assert_eq!(func(vec![1, 2, 4], 3), 6);
    }
    test(missing_element);
}
