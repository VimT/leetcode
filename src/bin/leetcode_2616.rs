//! 最小化数对的最大差值

pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut left = 0;
    let mut right = nums[len - 1];
    while left < right {
        let mid = (left + right) / 2;
        let mut i = 1;
        let mut cnt = 0;
        while i < len {
            if nums[i] - nums[i - 1] <= mid {
                cnt += 1;
                i += 1;
            }
            i += 1;
        }
        if cnt >= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, p: i32) -> i32) {
        assert_eq!(func(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(func(vec![4, 2, 1, 2], 1), 0);
    }
    test(minimize_max);
}
