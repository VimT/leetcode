//! 表现良好的最长时间段

use std::collections::HashMap;

/// 将大于 8 小时的一天记为 1 分，小于等于 8 小时的一天记为 -1 分
/// 前缀和，然后问题变成求 最长i-j  presum[j] - presum[i] > 0，用单调栈或者map解决
/// 注意不能二分：不满足单调性
pub fn longest_wpi(hours: Vec<i32>) -> i32 {
    let len = hours.len();
    let mut cur_sum = 0;
    let mut map = HashMap::new();
    let mut result = 0;
    for i in 0..len {
        cur_sum += if hours[i] > 8 { 1 } else { -1 };
        map.entry(cur_sum).or_insert(i);
        if cur_sum > 0 {
            result = i + 1;
        } else if let Some(&idx) = map.get(&(cur_sum - 1)) {
            result = result.max(i - idx);
        }
    }
    result as i32
}

/// 单调栈找 nums[j] > nums[i] 的最大区间长度
/// 观察可以发现一个重要性质：枚举最大区间长度的左端点，发现左端点是不断下降的。
pub fn longest_wpi2(hours: Vec<i32>) -> i32 {
    let len = hours.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + if hours[i] > 8 { 1 } else { -1 };
    }
    let mut s = vec![];
    // 顺序生成单调减序列
    for i in 0..=len {
        if s.is_empty() || presum[*s.last().unwrap()] > presum[i] {
            s.push(i);
        }
    }
    let mut result = 0;
    // 倒序扫描数组，求最大长度坡
    for i in (0..=len).rev() {
        while !s.is_empty() && presum[*s.last().unwrap()] < presum[i] {
            result = result.max(i as i32 - s.pop().unwrap() as i32);
        }
    }
    result
}


fn main() {
    fn test(func: fn(hours: Vec<i32>) -> i32) {
        assert_eq!(func(vec![9, 9, 6, 0, 6, 6, 9]), 3);
        assert_eq!(func(vec![6, 6, 6]), 0);
        assert_eq!(func(vec![6, 6, 6, 9]), 1);
        assert_eq!(func(vec![6, 9, 9]), 3);
        assert_eq!(func(vec![9, 6, 9]), 3);
    }
    test(longest_wpi);
    test(longest_wpi2);
}
