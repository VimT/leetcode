//! 形成目标数组的子数组最少增加次数

/// 对一个数num1，这个数本身一定要num1次，对于下一个数num2
/// 如果 num2 > num1 ，那么这个数需要 num2 - num1 次
/// 如果 num2 < num1 ，那在累加num1的时候就可以附带累积 num2 了
/// 所以结果就是两个数之间的差值
pub fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut result = target[0];
    for w in target.windows(2) {
        result += (w[1] - w[0]).max(0);
    }
    result
}

fn main() {
    fn test(func: fn(target: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 4, 2, 5, 6]), 8);
        assert_eq!(func(vec![1, 2, 3, 2, 1]), 3);
        assert_eq!(func(vec![3, 1, 1, 2]), 4);
        assert_eq!(func(vec![3, 1, 5, 4, 2]), 7);
        assert_eq!(func(vec![1, 1, 1, 1]), 1);
    }
    test(min_number_operations);
}
