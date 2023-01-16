//! 查询数组 Xor 美丽值

pub fn xor_beauty(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |a, b| a ^ b)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 4]), 5);
        assert_eq!(func(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30]), 34);
    }
    test(xor_beauty);
}
