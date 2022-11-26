//! 统计定界子数组的数目

/// 数组以不在[min_k, max_k]范围的点切分，对每个子数组，枚举右端点，左端点是 (split_point, last_min.max(last_max)]
pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut last_min = -1;
    let mut last_max = -1;
    let mut last_split = -1;
    let mut result = 0;
    for (i, num) in nums.into_iter().enumerate() {
        if num < min_k || num > max_k {
            last_split = i as i32;
            last_min = -1;
            last_max = -1;
            continue;
        }
        if num == min_k {
            last_min = i as i32;
        }
        if num == max_k {
            last_max = i as i32;
        }
        let end = last_min.min(last_max);
        if end != -1 {
            result += (end - last_split) as i64;
        }
    }

    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64) {
        assert_eq!(func(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
        assert_eq!(func(vec![1, 1, 1, 1], 1, 1), 10);
    }
    test(count_subarrays);
}
