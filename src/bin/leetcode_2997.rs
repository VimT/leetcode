//! 使数组异或和等于 K 的最少操作次数

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let num = nums.into_iter().reduce(|a, x| a ^ x).unwrap();
    let mut result = 0;
    for i in 0..30 {
        if num >> i & 1 != k >> i & 1 {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 1, 3, 4], 1), 2);
        assert_eq!(func(vec![2, 0, 2, 0], 0), 0);
    }
    test(min_operations);
}
