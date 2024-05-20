//! 特殊数组 I

pub fn is_array_special(nums: Vec<i32>) -> bool {
    nums.windows(2).all(|w| w[0] % 2 != w[1] % 2)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1]), true);
        assert_eq!(func(vec![2, 1, 4]), true);
        assert_eq!(func(vec![4, 3, 1, 6]), false);
    }
    test(is_array_special);
}
