//! 最小化数组中的最大值

pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = *nums.iter().max().unwrap() + 1;
    while left < right {
        let mid = (left + right) / 2;
        let mut more = 0;
        for &num in nums.iter().rev() {
            if num >= mid {
                more += (num - mid) as i64;
            } else {
                more = (more - (mid - num) as i64).max(0);
            }
        }
        if more == 0 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    assert_eq!(minimize_array_value(vec![3, 7, 1, 6]), 5);
    assert_eq!(minimize_array_value(vec![10, 1]), 10);
}
