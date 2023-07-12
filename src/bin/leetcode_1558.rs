//! 得到目标数组的最少函数调用次数

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut one = 0;
    let mut multi = 0;
    for num in nums {
        one += num.count_ones() as i32;
        multi = multi.max(31 - num.leading_zeros() as i32);
    }
    one + multi
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0]), 0);
        assert_eq!(func(vec![1, 5]), 5);
        assert_eq!(func(vec![2, 2]), 3);
        assert_eq!(func(vec![4, 2, 5]), 6);
        assert_eq!(func(vec![3, 2, 2, 4]), 7);
        assert_eq!(func(vec![2, 4, 8, 16]), 8);
    }
    test(min_operations);
}
