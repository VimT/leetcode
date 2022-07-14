//! 子数组最大平均数 II

/// 浮点数二分查找
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    const MIN: f64 = 1e-5;
    let mut min = nums[0];
    let mut max = nums[0];
    for &num in &nums {
        min = min.min(num);
        max = max.max(num);
    }
    let mut left = min as f64;
    let mut right = max as f64;
    let mut prev_mid = max as f64;
    let mut error = i32::MAX as f64;
    fn check(nums: &Vec<i32>, mid: f64, k: usize) -> bool {
        let mut sum = 0.;
        let mut prev = 0.;
        let mut min_sum = 0.0f64;
        for i in 0..k {
            sum += nums[i] as f64 - mid;
        }
        if sum >= 0. { return true; }
        for i in k..nums.len() {
            sum += nums[i] as f64 - mid;
            prev += nums[i - k] as f64 - mid;
            min_sum = min_sum.min(prev);
            // sum 表示 前i项和，min_sum表示前j项和里最小的， sum - min_sum 表示 j项到i项的和
            // 妙啊。这样就相当于 任意>k长度区间
            if sum >= min_sum {
                return true;
            }
        }
        false
    }
    while error > MIN {
        let mid = (left + right) * 0.5;
        if check(&nums, mid, k as usize) {
            left = mid;
        } else {
            right = mid;
        }
        error = (prev_mid - mid).abs();
        prev_mid = mid;
    }
    left
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> f64) {
        assert_eq!((func(vec![1, 12, -5, -6, 50, 3], 4) - 12.75000).abs() < 1e-5, true);
        assert_eq!(func(vec![5], 1), 5.00000);
    }
    test(find_max_average);
}
