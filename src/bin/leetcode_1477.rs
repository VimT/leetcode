//! 找两个和为目标值且不重叠的子数组

use std::collections::HashMap;

pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let mut cur = 0;
    let mut past: HashMap<i64, Vec<usize>> = HashMap::new();
    past.insert(0, vec![0]);
    let mut result = i32::MAX;
    let mut pre_min = vec![i32::MAX; arr.len() + 1];

    for (i, num) in arr.into_iter().enumerate() {
        pre_min[i + 1] = pre_min[i];
        cur += num as i64;
        if let Some(q) = past.get(&(cur - target as i64)) {
            let last = *q.last().unwrap();
            let len = (i + 1 - last) as i32;
            result = result.min(pre_min[last].saturating_add(len));
            pre_min[i + 1] = pre_min[i + 1].min(len);
        }
        past.entry(cur).or_default().push(i + 1);
    }
    if result == i32::MAX { -1 } else { result }
}


/// 双指针
pub fn min_sum_of_lengths2(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();
    let mut dp = vec![i32::MAX / 2; len + 1]; // 以 i 结尾组成的子数组满足target的长度
    let mut left = 0;
    let mut result = i32::MAX;
    let mut sum = 0;
    for right in 0..len {
        sum += arr[right];
        while sum > target {
            sum -= arr[left];
            left += 1;
        }
        if sum == target {
            result = result.min((right - left) as i32 + 1 + dp[left]);
            dp[right + 1] = dp[right].min((right - left) as i32 + 1);
        } else {
            dp[right + 1] = dp[right];
        }
    }
    if result >= i32::MAX / 2 { -1 } else { result }
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 6, 1], 7), -1);
        assert_eq!(func(vec![3, 2, 2, 4, 3], 3), 2);
        assert_eq!(func(vec![7, 3, 4, 7], 7), 2);
        assert_eq!(func(vec![4, 3, 2, 6, 2, 3, 4], 6), -1);
    }
    test(min_sum_of_lengths);
    test(min_sum_of_lengths2);
}
