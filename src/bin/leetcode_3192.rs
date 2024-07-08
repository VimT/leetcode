//! 使二进制数组全部等于 1 的最少操作次数 II

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut rev = 0;
    let mut result = 0;
    for i in 0..len {
        if nums[i] == rev {
            result += 1;
            rev = 1 - rev;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1, 1, 0, 1]), 4);
        assert_eq!(func(vec![1, 0, 0, 0]), 1);
    }
    test(min_operations);
}
