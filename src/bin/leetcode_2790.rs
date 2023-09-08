//! 长度递增组的最大数目

/// 二分
/// 想象成一个正方形的左下角，我们把数字从大到小竖着填充到这个左下角的三角形。
/// 如果这一列的数不够填充这个三角形，就等后面的多出来的数补。
pub fn max_increasing_groups(mut usage_limits: Vec<i32>) -> i32 {
    let len = usage_limits.len();
    usage_limits.sort_unstable();
    let mut left = 1;
    let mut right = len as i32;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut gap = 0;
        let mut n = mid;
        for &num in usage_limits.iter().rev() {
            gap = (gap + num - n).min(0);
            if n > 0 { n -= 1; }
        }
        if gap >= 0 {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

/// 对1的优化：从低往高堆就行
pub fn max_increasing_groups2(mut usage_limits: Vec<i32>) -> i32 {
    usage_limits.sort_unstable();
    let mut remain = 0; // 前面数组多余的
    let mut require = 1;  // 最小目标序列需要的长度
    for num in usage_limits {
        remain += num as i64;
        if remain >= require {
            remain -= require;
            require += 1;
        }
    }
    (require - 1) as i32
}

fn main() {
    fn test(func: fn(usage_limits: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1000000000, 999999999, 999999998]), 3);
        assert_eq!(func(vec![2, 2, 2]), 3);
        assert_eq!(func(vec![1, 2, 5]), 3);
        assert_eq!(func(vec![2, 1, 2]), 2);
        assert_eq!(func(vec![1, 1]), 1);
        assert_eq!(func(vec![1, 1, 5]), 2);
    }
    test(max_increasing_groups);
    test(max_increasing_groups2);
}
