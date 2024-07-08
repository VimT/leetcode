//! 使所有元素都可以被 3 整除的最少操作数

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.into_iter().map(|x| (x % 3 != 0) as i32).sum()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4]), 3);
        assert_eq!(func(vec![3, 6, 9]), 0);
    }
    test(minimum_operations);
}
