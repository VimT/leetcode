//! 统计得分小于 K 的子数组数目

pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    let mut result = 0;
    let mut i = 0;
    for j in 0..len {
        while i <= j && (presum[j + 1] - presum[i]) * ((j + 1 - i) as i64) >= k {
            i += 1;
        }
        result += j + 1 - i;
    }
    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i64) -> i64) {
        assert_eq!(func(vec![2, 1, 4, 3, 5], 10), 6);
        assert_eq!(func(vec![2, 1, 4, 3, 5], 2), 1);
        assert_eq!(func(vec![1, 1, 1], 5), 5);
    }
    test(count_subarrays);
}
