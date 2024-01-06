//! 执行操作使频率分数最大

/// 二分查找
pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut left = 0;
    let mut right = len;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut ok = false;
        for start in 0..=len - mid {
            let end = start + mid;
            let median = start + mid / 2;
            let right = presum[end] - presum[median] - (nums[median] as i64) * (end - median) as i64;
            let left = (nums[median] as i64) * (median - start + 1) as i64 - (presum[median + 1] - presum[start]);
            if left + right <= k {
                ok = true;
                break;
            }
        }
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

/// 滑动窗口
pub fn max_frequency_score2(mut nums: Vec<i32>, k: i64) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let distance = |start, end| {
        let median = start + (end - start) / 2;
        let right = presum[end] - presum[median] - (nums[median] as i64) * (end - median) as i64;
        let left = (nums[median] as i64) * (median - start + 1) as i64 - (presum[median + 1] - presum[start]);
        left + right
    };
    let mut result = 0;
    let mut left = 0;
    for right in 1..=len {
        while distance(left, right) > k {
            left += 1;
        }
        result = result.max(right - left);
    }
    result as i32
}


/// 贡献法优化空间复杂度
/// 移入窗口，对操作次数的贡献是 nums[i]
/// 变成正中间的数字，对操作次数的贡献是 0
/// 变成左半边的数字，对操作次数的贡献是 −nums[i]
pub fn max_frequency_score3(mut nums: Vec<i32>, k: i64) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    let mut result = 0;
    let mut left = 0;
    let mut sum = 0;
    for right in 1..=len {
        sum += nums[right - 1] as i64 - nums[(left + right - 1) / 2] as i64;
        while sum > k {
            sum += nums[left] as i64 - nums[(left + right) / 2] as i64;
            left += 1;
        }
        result = result.max(right - left);
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i64) -> i32) {
        assert_eq!(func(vec![1, 4, 4, 2, 4], 0), 3);
        assert_eq!(func(vec![1, 2, 6, 4], 3), 3);
    }
    test(max_frequency_score);
    test(max_frequency_score2);
    test(max_frequency_score3);
}
