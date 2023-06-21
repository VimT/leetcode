//! 最大或值

/// 贪心，一个数会用完所有左移次数
/// 这个题不能 dp
pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut suf = vec![0; len + 1];
    for i in (0..len).rev() {
        suf[i] = suf[i + 1] | nums[i];
    }
    let mut cur = 0;
    let mut result = 0;
    for i in 0..len {
        result = result.max(cur | (nums[i] as i64) << k | suf[i + 1] as i64);
        cur |= nums[i] as i64;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![10, 8, 4], 1), 30);
        assert_eq!(func(vec![12, 9], 1), 30);
        assert_eq!(func(vec![8, 1, 2], 2), 35);
    }
    test(maximum_or);
}
