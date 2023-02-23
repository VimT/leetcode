//! 打家劫舍 IV

pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = *nums.iter().max().unwrap();
    let len = nums.len();
    let mut dp = vec![0; len];
    while left < right {
        dp.fill(0);
        let mid = left + (right - left) / 2;
        for i in 0..len {
            if i > 0 { dp[i] = dp[i - 1]; }
            if nums[i] <= mid {
                dp[i] = dp[i].max(if i > 1 { dp[i - 2] } else { 0 } + 1);
            }
        }
        if dp[len - 1] >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

/// dp的空间优化，滚动变量
pub fn min_capability2(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = *nums.iter().max().unwrap();
    let len = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        let mut f0 = 0;
        let mut f1 = 0;
        for i in 0..len {
            let (of0, of1) = (f0, f1);
            if nums[i] <= mid {
                f1 = f1.max(of0 + 1);
            }
            f0 = of1;
        }
        if f1 >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 3, 5, 9], 2), 5);
        assert_eq!(func(vec![2, 7, 9, 3, 1], 2), 2);
    }
    test(min_capability);
    test(min_capability2);
}
