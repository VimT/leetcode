//! 超过阈值的最少操作数 I

pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    for (i, &num) in nums.iter().enumerate() {
        if num >= k {
            return i as i32;
        }
    }
    nums.len() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 11, 10, 1, 3], 10), 3);
        assert_eq!(func(vec![1, 1, 2, 4, 9], 1), 0);
        assert_eq!(func(vec![1, 1, 2, 4, 9], 9), 4);
    }
    test(min_operations);
}
