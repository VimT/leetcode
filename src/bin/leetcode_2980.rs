//! 检查按位或是否存在尾随零

pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    nums.into_iter().filter(|x| x & 1 == 0).count() > 1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(func(vec![2, 4, 8, 16]), true);
        assert_eq!(func(vec![1, 3, 5, 7, 9]), false);
    }
    test(has_trailing_zeros);
}
